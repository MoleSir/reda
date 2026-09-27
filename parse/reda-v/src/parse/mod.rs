mod base;
mod expr;
pub mod error;
pub use error::ParseError;
use std::path::Path;

use base::*;
use expr::*;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{branch::alt, bytes::complete::tag, character::complete::char, multi::separated_list0, sequence::delimited};
use nom::combinator::{map, opt};
use crate::model::{Always, Assign, Connection, EdgeKind, Instance, Module, Net, NetKind, Parameter, Port, PortDirection, Range, SensitivityEvent, SensitivityList, Stmt, Verilog};

pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Verilog, ParseError> {
    let source = std::fs::read_to_string(path)?;
    let (_, v) = verilog(&source).map_err(|e| ParseError::Parse(e.to_string()))?;
    Ok(v)
}

pub fn load_str(source: &str) -> Result<Verilog, ParseError> {
    let (_, v) = verilog(&source).map_err(|e| ParseError::Parse(e.to_string()))?;
    Ok(v)
}

pub fn verilog(input: &str) -> NomResult<Verilog> {
    let (input, modules) = many0(hws(module))(input)?;
    Ok((input, Verilog { modules }))
}

pub fn module(input: &str) -> NomResult<Module> {
    let (input, _) = hws(tag("module"))(input)?;
    let (input, name) = hws(identifier)(input).to_failure()?;
    let (input, port_names) = delimited(
        hws(char('(')),
        separated_list0(hws(char(',')), identifier),
        hws(char(')'))
    )(input).to_failure()?;
    let (input, _) = hws(char(';'))(input).to_failure()?;

    // parse items until endmodule
    let (input, items) = many0(alt((
        map(port, ModuleItem::Port),
        map(parameter, ModuleItem::Parameter),
        map(net, ModuleItem::Net),
        map(assign, ModuleItem::Assign),
        map(always, ModuleItem::Always),
        map(instance, ModuleItem::Instance),
    )))(input)?;

    let (input, _) = hws(tag("endmodule"))(input).to_failure()?;

    let mut ports = vec![];
    let mut parameters = vec![];
    let mut nets = vec![];
    let mut assigns = vec![];
    let mut alwayses = vec![];
    let mut instances = vec![];

    for item in items {
        match item {
            ModuleItem::Port(p) => ports.push(p),
            ModuleItem::Parameter(p) => parameters.push(p),
            ModuleItem::Net(n) => nets.push(n),
            ModuleItem::Assign(a) => assigns.push(a),
            ModuleItem::Always(a) => alwayses.push(a),
            ModuleItem::Instance(i) => instances.push(i),
        }
    }

    Ok((input, Module {
        name: name.to_string(),
        port_names: port_names.into_iter().map(|p| p.to_string()).collect(),
        ports,
        nets,
        parameters,
        assign: assigns,
        always: alwayses,
        instances,
    }))
}

enum ModuleItem {
    Port(Port),
    Parameter(Parameter),
    Net(Net),
    Assign(Assign),
    Always(Always),
    Instance(Instance),
}

pub fn port(input: &str) -> NomResult<Port> {
    let (input, dir) = hws(hws(alt((
        map(tag("input"), |_| PortDirection::Input),
        map(tag("output"), |_| PortDirection::Output),
        map(tag("inout"), |_| PortDirection::Inout),
    ))))(input)?;

    let (input, width) = opt(range)(input).to_failure()?;
    let (input, name) = hws(identifier)(input).to_failure()?;
    let (input, _) = hws(char(';'))(input).to_failure()?;

    Ok((input, Port {
        name: name.to_string(),
        direction: dir,
        width,
    }))
}

pub fn range(input: &str) -> NomResult<Range> {
    let (input, _) = hws(char('['))(input)?;
    let (input, msb) = unsigned_int(input).to_failure()?;
    let (input, _) = hws(char(':'))(input).to_failure()?;
    let (input, lsb) = unsigned_int(input).to_failure()?;
    let (input, _) = hws(char(']'))(input).to_failure()?;
    Ok((input, Range { msb: msb as i32, lsb: lsb as i32 }))
}

pub fn parameter(input: &str) -> NomResult<Parameter> {
    let (input, _) = hws(tag("parameter"))(input)?;
    let (input, name) = hws(identifier)(input)?;
    let (input, _) = hws(char('='))(input)?;
    let (input, expr) = hws(const_expr)(input)?;
    Ok((input, Parameter {
        name: name.to_string(),
        value: expr,
    }))
}

pub fn net(input: &str) -> NomResult<Net> {
    let (input, kind) = hws(alt((
        map(tag("wire"), |_| NetKind::Wire),
        map(tag("reg"), |_| NetKind::Reg),
    )))(input)?;
    let (input, width) = opt(range)(input).to_failure()?;
    let (input, name) = hws(identifier)(input).to_failure()?;
    let (input, _) = hws(char(';'))(input).to_failure()?;
    Ok((input, Net {
        kind,
        name: name.to_string(),
        width,
    }))
}

pub fn assign(input: &str) -> NomResult<Assign> {
    let (input, _) = hws(tag("assign"))(input)?;
    let (input, lhs) = var_expr(input).to_failure()?; 
    let (input, _) = hws(char('='))(input).to_failure()?;
    let (input, rhs) = expr(input).to_failure()?;
    let (input, _) = hws(char(';'))(input).to_failure()?;
    Ok((input, Assign { lhs, rhs }))
}

pub fn always(input: &str) -> NomResult<Always> {
    let (input, _) = hws(tag("always"))(input)?;
    let (input, _) = hws(char('@'))(input).to_failure()?;

    let (input, _) = hws(char('('))(input).to_failure()?;

    // * or event?
    let (input, width) = opt(hws(char('*')))(input).to_failure()?;
    let (input, list) = match width {
        Some(_) => (input, SensitivityList::Star),
        None => {
            let (input, events) = hws(separated_list0(hws(char(',')), sensitivity_event))(input)?;
            (input, SensitivityList::List(events))
        }
    };
    let (input, _) = hws(char(')'))(input).to_failure()?;
    let (input, stmts) = hws(stmts)(input).to_failure()?;

    Ok((input, Always {
        sensitivity: list,
        stmts, 
    }))
}

pub fn sensitivity_event(input: &str) -> NomResult<SensitivityEvent> {
    let (input, edge) = hws(opt(alt((
        map(tag("posedge"), |_| EdgeKind::Posedge),
        map(tag("negedge"), |_| EdgeKind::Negedge),
    ))))(input)?;

    let (input, signal) = hws(identifier)(input).to_failure()?;
    Ok((input, SensitivityEvent {
        edge,
        signal: signal.to_string(),
    }))
}

pub fn instance(input: &str) -> NomResult<Instance> {
    let (input, module_name) = hws(identifier)(input)?;
    let (input, instance_name) = hws(identifier)(input)?;

    let (input, connections) = delimited(
        hws(char('(')),
        separated_list0(
            hws(char(',')),
            connection, // 解析单个 .port(expr)
        ),
        hws(char(')')),
    )(input).to_failure()?;

    let (input, _) = hws(char(';'))(input).to_failure()?;

    Ok((
        input,
        Instance {
            module_name: module_name.to_string(),
            instance_name: instance_name.to_string(),
            connections,
        },
    ))
}

/// 解析单个端口连接：.port(expr)
pub fn connection(input: &str) -> NomResult<Connection> {
    let (input, _) = hws(char('.'))(input)?;
    let (input, port) = hws(identifier)(input)?;
    let (input, expr) = delimited(hws(char('(')), expr, hws(char(')')))(input)?;
    Ok((input, Connection {
        port: port.to_string(),
        expr,
    }))
}

pub fn stmts(input: &str) -> NomResult<Vec<Stmt>> {
    /*
        stmts := "begin" stmt* "end"
              | stmt
    */
    let parse_block = delimited(
        hws(tag("begin")),
        many0(hws(stmt)),
        hws(tag("end")),
    );
    alt((
        parse_block,
        map(hws(stmt), |s| vec![s]),
    ))(input)
}

pub fn stmt(input: &str) -> NomResult<Stmt> {
    // TODO: case
    alt((   
        blocking_assign_stmt,
        nonblocking_assign_stmt,
        if_stmt,
    ))(input)
}

pub fn blocking_assign_stmt(input: &str) -> NomResult<Stmt> {
    let (input, lhs) = hws(var_expr)(input)?;
    let (input, _) = hws(tag("="))(input)?;
    let (input, rhs) = hws(expr)(input)?;
    let (input, _) = hws(char(';'))(input)?;
    Ok((input, Stmt::BlockingAssign { lhs, rhs }))
}
    
pub fn nonblocking_assign_stmt(input: &str) -> NomResult<Stmt> {
    let (input, lhs) = hws(var_expr)(input)?;
    let (input, _) = hws(tag("<="))(input)?;
    let (input, rhs) = hws(expr)(input)?;
    let (input, _) = hws(char(';'))(input)?;
    Ok((input, Stmt::NonBlockingAssign { lhs, rhs }))
}

pub fn if_stmt(input: &str) -> NomResult<Stmt> {
    let (input, _) = hws(tag("if"))(input)?;
    let (input, cond) = delimited(hws(char('(')), expr, hws(char(')')))(input)?;
    let (input, then_body) = stmts(input)?;
    // optional else
    let (input, else_body) = opt(preceded(hws(tag("else")), stmts))(input)?;
    Ok((input, Stmt::If { cond, then_body, else_body }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{EdgeKind, Expr, NetKind, PortDirection, ValueBase, VarExpr};

    #[test]
    fn test_empty_module() {
        let input = r#"
module mymod (a, b, c);
endmodule
"#;
        let (_, m) = module(input).unwrap();
        assert_eq!(m.name, "mymod");
        assert_eq!(m.port_names.len(), 3);
    }

    #[test]
    fn test_port_parsing() {
        let inputs = vec![
            ("input clk;", "clk", PortDirection::Input),
            ("output [7:0] data;", "data", PortDirection::Output),
            ("inout rst;", "rst", PortDirection::Inout),
        ];

        for (src, name, dir) in inputs {
            let (_, p) = port(src).unwrap();
            assert_eq!(p.name, name);
            assert_eq!(p.direction, dir);
        }
    }

    #[test]
    fn test_range_parsing() {
        let input = "[15:0]";
        let (_, r) = range(input).unwrap();
        assert_eq!(r.msb, 15);
        assert_eq!(r.lsb, 0);
    }

    #[test]
    fn test_net_parsing() {
        let cases = vec![
            ("wire clk;", "clk", NetKind::Wire),
            ("reg [3:0] data;", "data", NetKind::Reg),
        ];

        for (src, name, kind) in cases {
            let (_, n) = net(src).unwrap();
            assert_eq!(n.name, name);
            assert_eq!(n.kind, kind);
        }
    }

    #[test]
    fn test_assign_parsing() {
        let input = "assign c = a;";
        let (_, a) = assign(input).unwrap();
        match a.lhs {
            VarExpr::Ident(ref s) => assert_eq!(s, "c"),
            _ => panic!("LHS not identifier"),
        }
        match a.rhs {
            Expr::Ident(ref s) => assert_eq!(s, "a"),
            _ => panic!("RHS not identifier"),
        }
    }

    #[test]
    fn test_sensitivity_event_parsing() {
        let inputs = vec![
            ("posedge clk", Some(EdgeKind::Posedge), "clk"),
            ("negedge rst", Some(EdgeKind::Negedge), "rst"),
            ("data", None, "data"),
        ];

        for (src, edge, signal) in inputs {
            let (_, e) = sensitivity_event(src).unwrap();
            assert_eq!(e.edge, edge);
            assert_eq!(e.signal, signal);
        }
    }

    #[test]
    fn test_instance_parsing() {
        let input = r#"
my_module u1 (
    .a(signal1),
    .b(signal2),
    .c(signal3)
);
"#;
        let (_, inst) = instance(input).unwrap();
        assert_eq!(inst.module_name, "my_module");
        assert_eq!(inst.instance_name, "u1");
        assert_eq!(inst.connections.len(), 3);
        assert_eq!(inst.connections[0].port, "a");
        match inst.connections[0].expr {
            Expr::Ident(ref s) => assert_eq!(s, "signal1"),
            _ => panic!("Expr not identifier"),
        }
    }

    #[test]
    fn test_stmts_single() {
        let input = "a = 1;";
        let (_, ss) = stmts(input).unwrap();
        assert_eq!(ss.len(), 1);
    }
    
    #[test]
    fn test_stmts_block() {
        let input = r#"
        begin
            a = 1;
            b <= 2;
        end
        "#;
        let (_, ss) = stmts(input).unwrap();
        assert_eq!(ss.len(), 2);
    }

    #[test]
    fn test_always_star() {
        let input = r#"
        always @(*) begin
            a = b ;
            c <= d;
        end
        "#;
        let (_, alw) = always(input).unwrap();
        match alw.sensitivity {
            SensitivityList::Star => {}
            _ => panic!("expected star sensitivity"),
        }
        assert_eq!(alw.stmts.len(), 2);
    }

    #[test]
    fn test_always_posedge() {
        let input = r#"
        always @(posedge clk) q <= d;
        "#;
        let (_, alw) = always(input).unwrap();
        match &alw.sensitivity {
            SensitivityList::List(events) => {
                assert_eq!(events.len(), 1);
                assert_eq!(events[0].signal, "clk");
                assert_eq!(events[0].edge, Some(EdgeKind::Posedge));
            }
            _ => panic!("expected list sensitivity"),
        }
        assert_eq!(alw.stmts.len(), 1);
    }

    #[test]
    fn test_always_negedge_or_posedge() {
        let input = r#"
        always @(negedge rst, posedge clk) begin
            if (rst) q <= 0;
            else q <= d;
        end
        "#;
        let (_, alw) = always(input).unwrap();
        match &alw.sensitivity {
            SensitivityList::List(events) => {
                assert_eq!(events.len(), 2);
                assert_eq!(events[0].signal, "rst");
                assert_eq!(events[0].edge, Some(EdgeKind::Negedge));
                assert_eq!(events[1].signal, "clk");
                assert_eq!(events[1].edge, Some(EdgeKind::Posedge));
            }
            _ => panic!("expected list sensitivity"),
        }
        assert_eq!(alw.stmts.len(), 1); // if 语句
    }

    #[test]
    fn test_always_simple_stmt() {
        let input = r#"
        always @(posedge clk) a <= b;
        "#;
        let (_, alw) = always(input).unwrap();
        assert_eq!(alw.stmts.len(), 1);
    }

    #[test]
    fn test_always_if_else() {
        let input = r#"
        always @(*) begin
            if (a) b = 1;
            else b = 0;
        end
        "#;
        let (_, alw) = always(input).unwrap();
        assert_eq!(alw.stmts.len(), 1); // if stmt
    }

    #[test]
    fn test_bin_value_bits() {
        assert_eq!(
            bin_value_bits("1010").unwrap().1,
            "1010".to_string()
        );
        assert_eq!(
            bin_value_bits("10xz_01z1").unwrap().1,
            "10xz_01z1".to_string()
        );
    }

    #[test]
    fn test_dec_value_bits() {
        assert_eq!(
            dec_value_bits("255").unwrap().1,
            "255".to_string()
        );
        assert_eq!(
            dec_value_bits("12_345").unwrap().1,
            "12_345".to_string()
        );
    }

    #[test]
    fn test_oct_value_bits() {
        assert_eq!(
            oct_value_bits("765").unwrap().1,
            "765".to_string()
        );
        assert_eq!(
            oct_value_bits("7xz_1").unwrap().1,
            "7xz_1".to_string()
        );
    }

    #[test]
    fn test_hex_value_bits() {
        assert_eq!(
            hex_value_bits("FF").unwrap().1,
            "FF".to_string()
        );
        assert_eq!(
            hex_value_bits("A3c_zX").unwrap().1,
            "A3c_zX".to_string()
        );
    }

    #[test]
    fn test_value_parser() {
        // 4'b1010
        let (_, v) = value("4'b1010").unwrap();
        assert_eq!(v.len, 4);
        assert_eq!(v.base, ValueBase::Bin);
        assert_eq!(v.bits, "1010");

        // 16'd255
        let (_, v) = value("16'd255").unwrap();
        assert_eq!(v.len, 16);
        assert_eq!(v.base, ValueBase::Dec);
        assert_eq!(v.bits, "255");

        // 8'hFF
        let (_, v) = value("8'hFF").unwrap();
        assert_eq!(v.len, 8);
        assert_eq!(v.base, ValueBase::Hex);
        assert_eq!(v.bits, "FF");

        // 8'b10xz_01z1
        let (_, v) = value("8'b10xz_01z1").unwrap();
        assert_eq!(v.len, 8);
        assert_eq!(v.base, ValueBase::Bin);
        assert_eq!(v.bits, "10xz_01z1");
    }


    #[test]
    fn test_parameter_simple_value() {
        let (_, p) = parameter("parameter WIDTH = 8").unwrap();
        assert_eq!(p.name, "WIDTH");
    }

    #[test]
    fn test_parameter_with_hex() {
        let (_, p) = parameter("parameter ADDR = 16'hFF").unwrap();
        assert_eq!(p.name, "ADDR");
    }

    #[test]
    fn test_parameter_with_expr() {
        let (_, p) = parameter("parameter SIZE = 4*1024").unwrap();
        assert_eq!(p.name, "SIZE");
    }
}
