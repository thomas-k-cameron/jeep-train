use crate::Conn;

type FuncN = fn (conn: Conn) -> ();
type FuncB = fn (conn: Conn) -> bool;

pub type PluginType = FuncN;
pub type RouterType = FuncB;