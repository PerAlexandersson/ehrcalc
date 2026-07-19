//! Flow-polytope lattice-point counts and Ehrhart h*-vectors.
//!
//! We use the standard acyclic directed graph model.  For a directed graph
//! `G=(V,E)` and an integral netflow vector `a` with sum zero, the flow polytope
//! is
//!
//! ```text
//! F_G(a) = { x_e >= 0 : out_x(v) - in_x(v) = a_v for all v }.
//! ```
//!
//! Lattice points in `t F_G(a)` are nonnegative integral flows with netflow
//! `t a`.  The counter below processes vertices in topological order and keeps
//! only the accumulated inflow at future vertices.

use crate::ehrhart::EhrhartPoly;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_rational::BigRational;
use num_traits::{One, Zero};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowPolytope {
    vertices: usize,
    edges: Vec<(usize, usize)>,
    netflow: Vec<i64>,
}

impl FlowPolytope {
    pub fn new(
        vertices: usize,
        edges: Vec<(usize, usize)>,
        netflow: Vec<i64>,
    ) -> Result<Self, String> {
        if vertices == 0 {
            return Err("flow polytope needs at least one vertex".to_string());
        }
        if netflow.len() != vertices {
            return Err(format!(
                "netflow has length {}, expected {vertices}",
                netflow.len()
            ));
        }
        let netflow_sum: i64 = netflow.iter().sum();
        if netflow_sum != 0 {
            return Err(format!(
                "netflow entries must sum to zero, got {netflow_sum}"
            ));
        }
        for &(tail, head) in &edges {
            if tail >= vertices || head >= vertices {
                return Err(format!(
                    "edge {} -> {} is outside 0..{}",
                    tail,
                    head,
                    vertices.saturating_sub(1)
                ));
            }
            if tail == head {
                return Err(format!("loop edge {} -> {} is not allowed", tail, head));
            }
        }

        Ok(Self {
            vertices,
            edges,
            netflow,
        })
    }

    pub fn vertices(&self) -> usize {
        self.vertices
    }

    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    pub fn netflow(&self) -> &[i64] {
        &self.netflow
    }

    /// Dimension of the smallest face containing `F_G(a)`.
    ///
    /// For an acyclic graph, every feasible integral flow decomposes into
    /// source-to-sink paths.  Hence an edge can be positive in the relative
    /// interior only if it is reachable from a positive-netflow vertex and can
    /// reach a negative-netflow vertex.  The dimension is then
    /// `|E_support| - rank(B_support)`.
    pub fn dimension(&self) -> Result<usize, String> {
        let graph = self.topological_data()?;
        let supported = self.supported_edges(&graph);
        let edge_count = supported.iter().filter(|&&x| x).count();
        if edge_count == 0 {
            return Ok(0);
        }

        let mut used_vertex = vec![false; self.vertices];
        let mut undirected = vec![Vec::new(); self.vertices];
        for (idx, &(tail, head)) in self.edges.iter().enumerate() {
            if !supported[idx] {
                continue;
            }
            used_vertex[tail] = true;
            used_vertex[head] = true;
            undirected[tail].push(head);
            undirected[head].push(tail);
        }

        let vertex_count = used_vertex.iter().filter(|&&x| x).count();
        let mut components = 0usize;
        let mut seen = vec![false; self.vertices];
        for start in 0..self.vertices {
            if !used_vertex[start] || seen[start] {
                continue;
            }
            components += 1;
            let mut queue = VecDeque::from([start]);
            seen[start] = true;
            while let Some(v) = queue.pop_front() {
                for &u in &undirected[v] {
                    if !seen[u] {
                        seen[u] = true;
                        queue.push_back(u);
                    }
                }
            }
        }

        Ok(edge_count + components - vertex_count)
    }

    /// Count lattice points in `dilation * F_G(a)`.
    pub fn count_lattice_points(
        &self,
        dilation: u64,
        max_states: Option<usize>,
    ) -> Result<BigUint, String> {
        let graph = self.topological_data()?;
        let dilation_i64 = i64::try_from(dilation)
            .map_err(|_| format!("dilation {dilation} is too large for i64 arithmetic"))?;
        let netflow = self
            .netflow
            .iter()
            .map(|&x| {
                x.checked_mul(dilation_i64)
                    .ok_or_else(|| format!("netflow entry {x} overflows at dilation {dilation}"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut ctx = CountContext {
            order: graph.order,
            outgoing_heads: graph.outgoing_heads,
            netflow,
            memo: HashMap::new(),
            max_states,
        };
        let incoming = vec![0i64; self.vertices];
        ctx.count_from(0, incoming)
    }

    /// Count lattice points in the relative interior of `dilation * F_G(a)`.
    ///
    /// If `S` is the support graph of the minimal face containing the polytope,
    /// relative interior means `x_e > 0` for every edge in `S` and `x_e = 0`
    /// outside `S`.  Setting `x_e = y_e + 1` for `e in S` reduces this to a
    /// nonnegative flow count with shifted netflow
    /// `dilation * a_v - (outdeg_S(v) - indeg_S(v))`.
    pub fn count_interior_lattice_points(
        &self,
        dilation: u64,
        max_states: Option<usize>,
    ) -> Result<BigUint, String> {
        let graph = self.topological_data()?;
        let supported = self.supported_edges(&graph);

        let mut support_edges = Vec::new();
        let mut support_balance = vec![0i64; self.vertices];
        for (idx, &(tail, head)) in self.edges.iter().enumerate() {
            if !supported[idx] {
                continue;
            }
            support_edges.push((tail, head));
            support_balance[tail] += 1;
            support_balance[head] -= 1;
        }

        let dilation_i64 = i64::try_from(dilation)
            .map_err(|_| format!("dilation {dilation} is too large for i64 arithmetic"))?;
        let shifted_netflow = self
            .netflow
            .iter()
            .zip(support_balance.iter())
            .map(|(&a_v, &b_v)| {
                a_v.checked_mul(dilation_i64)
                    .and_then(|x| x.checked_sub(b_v))
                    .ok_or_else(|| {
                        format!("shifted interior netflow overflows at dilation {dilation}")
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;

        FlowPolytope::new(self.vertices, support_edges, shifted_netflow)?
            .count_lattice_points(1, max_states)
    }

    /// Interpolate the Ehrhart polynomial using Ehrhart-Macdonald reciprocity.
    ///
    /// Negative evaluations are obtained from relative-interior flow counts:
    /// `P(-t) = (-1)^d |(t P)° ∩ Z^E|`.
    pub fn ehrhart_poly(&self, max_states: Option<usize>) -> Result<EhrhartPoly, String> {
        self.ehrhart_poly_with_reciprocity(max_states)
    }

    pub fn ehrhart_poly_with_reciprocity(
        &self,
        max_states: Option<usize>,
    ) -> Result<EhrhartPoly, String> {
        let dimension = self.dimension()?;
        if self.count_lattice_points(1, max_states)?.is_zero() {
            return Err("flow polytope is empty: no lattice point at dilation 1".to_string());
        }
        if dimension == 0 {
            return Ok(EhrhartPoly {
                coeffs: vec![BigRational::one()],
                degree: 0,
            });
        }

        let sign_pos = dimension % 2 == 0;
        let mut points: Vec<(i64, BigRational)> = Vec::with_capacity(dimension + 1);
        points.push((0, BigRational::one()));

        let mut pos_t = 0u64;
        let mut neg_t = 0u64;
        let mut last_pos_count = BigUint::one();
        let mut last_neg_count = BigUint::one();

        while points.len() <= dimension {
            if last_neg_count <= last_pos_count {
                neg_t += 1;
                let interior = self.count_interior_lattice_points(neg_t, max_states)?;
                let p_val = BigRational::from(
                    interior
                        .to_bigint()
                        .expect("BigUint to BigInt conversion cannot fail"),
                );
                points.push((-(neg_t as i64), if sign_pos { p_val } else { -p_val }));
                last_neg_count = interior;
            } else {
                pos_t += 1;
                let count = self.count_lattice_points(pos_t, max_states)?;
                let p_val = BigRational::from(
                    count
                        .to_bigint()
                        .expect("BigUint to BigInt conversion cannot fail"),
                );
                points.push((pos_t as i64, p_val));
                last_pos_count = count;
            }
        }

        Ok(poly_from_points(&points))
    }

    /// Interpolate the Ehrhart polynomial from exact positive-dilation counts.
    pub fn ehrhart_poly_positive(&self, max_states: Option<usize>) -> Result<EhrhartPoly, String> {
        let dimension = self.dimension()?;
        if self.count_lattice_points(1, max_states)?.is_zero() {
            return Err("flow polytope is empty: no lattice point at dilation 1".to_string());
        }

        let points = (0..=dimension)
            .map(|t| {
                let count = self.count_lattice_points(t as u64, max_states)?;
                let y = BigRational::from(
                    count
                        .to_bigint()
                        .expect("BigUint to BigInt conversion cannot fail"),
                );
                Ok((t as i64, y))
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(poly_from_points(&points))
    }

    fn supported_edges(&self, graph: &TopologicalData) -> Vec<bool> {
        let mut reachable_from_source = vec![false; self.vertices];
        let mut queue = VecDeque::new();
        for (v, &a_v) in self.netflow.iter().enumerate() {
            if a_v > 0 {
                reachable_from_source[v] = true;
                queue.push_back(v);
            }
        }
        while let Some(v) = queue.pop_front() {
            for &head in &graph.outgoing_heads[v] {
                if !reachable_from_source[head] {
                    reachable_from_source[head] = true;
                    queue.push_back(head);
                }
            }
        }

        let mut reaches_sink = vec![false; self.vertices];
        let mut queue = VecDeque::new();
        for (v, &a_v) in self.netflow.iter().enumerate() {
            if a_v < 0 {
                reaches_sink[v] = true;
                queue.push_back(v);
            }
        }
        while let Some(v) = queue.pop_front() {
            for &tail in &graph.incoming_tails[v] {
                if !reaches_sink[tail] {
                    reaches_sink[tail] = true;
                    queue.push_back(tail);
                }
            }
        }

        self.edges
            .iter()
            .map(|&(tail, head)| reachable_from_source[tail] && reaches_sink[head])
            .collect()
    }

    fn topological_data(&self) -> Result<TopologicalData, String> {
        let mut indegree = vec![0usize; self.vertices];
        let mut outgoing_heads = vec![Vec::new(); self.vertices];
        let mut incoming_tails = vec![Vec::new(); self.vertices];
        for &(tail, head) in &self.edges {
            indegree[head] += 1;
            outgoing_heads[tail].push(head);
            incoming_tails[head].push(tail);
        }

        let mut queue = VecDeque::new();
        for (v, &deg) in indegree.iter().enumerate() {
            if deg == 0 {
                queue.push_back(v);
            }
        }

        let mut order = Vec::with_capacity(self.vertices);
        while let Some(v) = queue.pop_front() {
            order.push(v);
            for &head in &outgoing_heads[v] {
                indegree[head] -= 1;
                if indegree[head] == 0 {
                    queue.push_back(head);
                }
            }
        }

        if order.len() != self.vertices {
            return Err("flow counting currently requires an acyclic directed graph".to_string());
        }

        Ok(TopologicalData {
            order,
            outgoing_heads,
            incoming_tails,
        })
    }
}

#[derive(Debug)]
struct TopologicalData {
    order: Vec<usize>,
    outgoing_heads: Vec<Vec<usize>>,
    incoming_tails: Vec<Vec<usize>>,
}

struct CountContext {
    order: Vec<usize>,
    outgoing_heads: Vec<Vec<usize>>,
    netflow: Vec<i64>,
    memo: HashMap<(usize, Vec<i64>), BigUint>,
    max_states: Option<usize>,
}

impl CountContext {
    fn count_from(&mut self, pos: usize, incoming: Vec<i64>) -> Result<BigUint, String> {
        if pos == self.order.len() {
            return Ok(if incoming.iter().all(|&x| x == 0) {
                BigUint::one()
            } else {
                BigUint::zero()
            });
        }

        let key = (pos, incoming.clone());
        if let Some(value) = self.memo.get(&key) {
            return Ok(value.clone());
        }

        if let Some(limit) = self.max_states {
            if self.memo.len() >= limit {
                return Err(format!("flow DP exceeded max state limit {limit}"));
            }
        }

        let vertex = self.order[pos];
        let total_out = incoming[vertex] + self.netflow[vertex];
        let result = if total_out < 0 {
            BigUint::zero()
        } else {
            let mut next_incoming = incoming;
            next_incoming[vertex] = 0;
            let heads = self.outgoing_heads[vertex].clone();
            if heads.is_empty() {
                if total_out == 0 {
                    self.count_from(pos + 1, next_incoming)?
                } else {
                    BigUint::zero()
                }
            } else {
                let mut total = BigUint::zero();
                self.distribute_outflow(pos, &heads, 0, total_out, &mut next_incoming, &mut total)?;
                total
            }
        };

        self.memo.insert(key, result.clone());
        Ok(result)
    }

    fn distribute_outflow(
        &mut self,
        pos: usize,
        heads: &[usize],
        edge_pos: usize,
        remaining: i64,
        incoming: &mut Vec<i64>,
        total: &mut BigUint,
    ) -> Result<(), String> {
        if edge_pos + 1 == heads.len() {
            let head = heads[edge_pos];
            incoming[head] += remaining;
            *total += self.count_from(pos + 1, incoming.clone())?;
            incoming[head] -= remaining;
            return Ok(());
        }

        let head = heads[edge_pos];
        for amount in 0..=remaining {
            incoming[head] += amount;
            self.distribute_outflow(
                pos,
                heads,
                edge_pos + 1,
                remaining - amount,
                incoming,
                total,
            )?;
            incoming[head] -= amount;
        }
        Ok(())
    }
}

fn interpolate(points: &[(i64, BigRational)]) -> Vec<BigRational> {
    let d = points.len();
    let mut mat: Vec<Vec<BigRational>> = points
        .iter()
        .map(|&(x, ref y)| {
            let xb = BigInt::from(x);
            let mut row = Vec::with_capacity(d + 1);
            let mut power = BigInt::one();
            for _ in 0..d {
                row.push(BigRational::from(power.clone()));
                power *= &xb;
            }
            row.push(y.clone());
            row
        })
        .collect();

    for col in 0..d {
        let pivot_row = (col..d)
            .find(|&r| !mat[r][col].is_zero())
            .expect("flow Ehrhart interpolation: singular system");
        mat.swap(col, pivot_row);
        let pivot = mat[col][col].clone();
        for j in col..=d {
            mat[col][j] = mat[col][j].clone() / &pivot;
        }
        for row in 0..d {
            if row == col {
                continue;
            }
            let factor = mat[row][col].clone();
            if factor.is_zero() {
                continue;
            }
            for j in col..=d {
                let sub = factor.clone() * &mat[col][j];
                mat[row][j] -= sub;
            }
        }
    }

    mat.into_iter().map(|row| row[d].clone()).collect()
}

fn poly_from_points(points: &[(i64, BigRational)]) -> EhrhartPoly {
    let coeffs = interpolate(points);
    let true_degree = coeffs
        .iter()
        .enumerate()
        .rev()
        .find(|(_, c)| !c.is_zero())
        .map(|(i, _)| i)
        .unwrap_or(0);
    EhrhartPoly {
        coeffs,
        degree: true_degree,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ehrhart::compute_hstar;

    #[test]
    fn single_edge_flow_polytope_is_point() {
        let flow = FlowPolytope::new(2, vec![(0, 1)], vec![1, -1]).unwrap();
        assert_eq!(flow.dimension().unwrap(), 0);
        assert_eq!(flow.count_lattice_points(7, None).unwrap(), BigUint::one());
        assert_eq!(
            flow.count_interior_lattice_points(7, None).unwrap(),
            BigUint::one()
        );

        let poly = flow.ehrhart_poly(None).unwrap();
        assert_eq!(poly.degree, 0);
        assert_eq!(compute_hstar(&poly), vec![BigInt::one()]);
    }

    #[test]
    fn diamond_flow_polytope_is_unit_segment() {
        let flow =
            FlowPolytope::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)], vec![1, 0, 0, -1]).unwrap();
        assert_eq!(flow.dimension().unwrap(), 1);
        assert_eq!(
            flow.count_lattice_points(5, None).unwrap(),
            BigUint::from(6u32)
        );
        assert_eq!(
            flow.count_interior_lattice_points(5, None).unwrap(),
            BigUint::from(4u32)
        );

        let poly = flow.ehrhart_poly(None).unwrap();
        assert_eq!(poly.degree, 1);
        assert_eq!(poly.eval(5), BigRational::from(BigInt::from(6)));
        assert_eq!(compute_hstar(&poly), vec![BigInt::one(), BigInt::zero()]);
    }

    #[test]
    fn parallel_edges_count_as_distinct_flow_variables() {
        let flow = FlowPolytope::new(2, vec![(0, 1), (0, 1)], vec![1, -1]).unwrap();
        assert_eq!(flow.dimension().unwrap(), 1);
        assert_eq!(
            flow.count_lattice_points(4, None).unwrap(),
            BigUint::from(5u32)
        );
    }

    #[test]
    fn reciprocity_matches_positive_interpolation_for_complete_dag_four() {
        let flow = FlowPolytope::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
            vec![1, 0, 0, -1],
        )
        .unwrap();
        let reciprocal = flow.ehrhart_poly_with_reciprocity(None).unwrap();
        let positive = flow.ehrhart_poly_positive(None).unwrap();

        assert_eq!(reciprocal.degree, 3);
        assert_eq!(reciprocal.coeffs, positive.coeffs);
        assert_eq!(reciprocal.eval(3), BigRational::from(BigInt::from(20)));
    }
}
