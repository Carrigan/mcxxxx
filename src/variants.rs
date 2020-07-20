enum XBusError {
  DataNotAvailable
}

trait XBus {
  fn read(&self) -> Result<Integer, XBusError>;
  fn write(&self, i: Integer);
}

trait SimpleIO {
  fn read(&self) -> Integer;
  fn write(&self, i: Integer);
}

struct MC4000 {
  p0: &dyn SimpleIO,
  p1: &dyn SimpleIO,
  x0: &dyn XBus,
  x1: &dyn XBus
}
