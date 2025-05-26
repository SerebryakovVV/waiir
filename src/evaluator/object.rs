// TODO: maybe will add objecttype and inspect, maybe not
#[derive(Debug, PartialEq)]
pub enum Object {
  INT(i32),
  BOOLEAN(bool),
  NULL
}
