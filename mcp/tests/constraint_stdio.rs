//! Real JSON-RPC stdio tool registration, schema, success and failure contracts.
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

struct Server {
    child: Child,
    input: ChildStdin,
    replies: Receiver<Value>,
}
impl Server {
    fn new() -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_ehrcalc-mcp"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = child.stdout.take().unwrap();
        let (tx, replies) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                let Ok(line) = line else {
                    break;
                };
                if let Ok(value) = serde_json::from_str(&line) {
                    if tx.send(value).is_err() {
                        break;
                    }
                }
            }
        });
        let mut server = Self {
            child,
            input,
            replies,
        };
        server.call(1, "initialize", json!({"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"constraint-test","version":"1"}}));
        writeln!(
            server.input,
            "{}",
            json!({"jsonrpc":"2.0","method":"notifications/initialized"})
        )
        .unwrap();
        server.input.flush().unwrap();
        server
    }
    fn call(&mut self, id: u64, method: &str, params: Value) -> Value {
        writeln!(
            self.input,
            "{}",
            json!({"jsonrpc":"2.0","id":id,"method":method,"params":params})
        )
        .unwrap();
        self.input.flush().unwrap();
        loop {
            let reply = self
                .replies
                .recv_timeout(Duration::from_secs(15))
                .expect("MCP response within 15 seconds");
            if reply["id"] == id {
                return reply;
            }
        }
    }
    fn tool(&mut self, id: u64, arguments: Value) -> Value {
        self.call(
            id,
            "tools/call",
            json!({"name":"constraint_polytope_ehrhart","arguments":arguments}),
        )
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn constraint_tool_is_registered_and_calls_the_shared_service() {
    let mut server = Server::new();
    let list = server.call(2, "tools/list", json!({}));
    let tool = list["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == "constraint_polytope_ehrhart")
        .unwrap();
    for tool in list["result"]["tools"].as_array().unwrap() {
        assert_eq!(tool["outputSchema"]["type"], "object");
    }
    assert!(tool["inputSchema"]["properties"]["input"].is_object());
    assert_eq!(tool["inputSchema"]["required"], json!(["input"]));
    let args = json!({"input":{"variables":1,"bounds":[[0,2]]}});
    let result = server.tool(3, args.clone());
    assert!(result.get("error").is_none(), "{result}");
    let value = &result["result"]["structuredContent"];
    let request = serde_json::from_value(args).unwrap();
    assert_eq!(
        value,
        &ehrcalc_mcp::execute_constraint_polytope(&request).unwrap()
    );
    assert_eq!(value["schema"], "ehrcalc.constraint-polytope-result.v1");
    assert_eq!(value["ehrhart_power"], json!(["1", "2"]));
    let existing = server.call(30, "tools/call", json!({"name":"compute_family","arguments":{"family":"kostka","lambda":[2,1],"weight":[1,1,1]}}));
    assert_eq!(existing["result"]["structuredContent"]["value"], "2");
    let warning = server.tool(4, json!({"input":{"variables":1,"bounds":[[0,"1/2"]]}}));
    assert!(!warning["result"]["structuredContent"]["warnings"]
        .as_array()
        .unwrap()
        .is_empty());
    let plan = server.tool(5, json!({"input":{"variables":1,"bounds":[[0,1]]},"plan_only":true,"order":[0],"order_heuristic":"rcm"}));
    assert_eq!(plan["result"]["structuredContent"]["samples"], json!([]));
    for (id, args) in [
        (6, json!({"input":{"variables":1,"bounds":[[2,1]]}})),
        (
            7,
            json!({"input":{"variables":1,"bounds":[[0,1]]},"order_heuristic":"bogus"}),
        ),
        (
            8,
            json!({"input":{"variables":1,"bounds":[[0,1]]},"max_states":0}),
        ),
        (
            9,
            json!({"input":{"variables":1,"bounds":[[0,1]]},"unexpected":true}),
        ),
    ] {
        let reply = server.tool(id, args.clone());
        assert!(
            reply.get("error").is_some() || reply["result"]["isError"] == true,
            "{reply}"
        );
        let repeated = server.tool(id + 10, args);
        assert_eq!(reply.get("error"), repeated.get("error"));
        assert_eq!(reply.get("result"), repeated.get("result"));
    }
}
