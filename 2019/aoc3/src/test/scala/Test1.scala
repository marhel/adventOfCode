import org.junit.Test
import org.junit.Assert._

class TestWires {
  @Test def parseWire(): Unit = {
    assertEquals(
      Seq(Direction.Right(8), Direction.Up(5), Direction.Left(4), Direction.Down(2)),
      Wires.parseWire("R8,U5,L4,D2"))
  }
  @Test def toWire(): Unit = {
    assertEquals(
      Wire(Seq(WireSegment(0, 0, 8, 0), WireSegment(8, 0, 8, 5), WireSegment(8, 5, 4, 5), WireSegment(4, 5, 4, 3))),
      Wires.toWire(Seq(Direction.Right(8), Direction.Up(5), Direction.Left(4), Direction.Down(2))))
  }
  @Test def toCoordinates(): Unit = {
    assertEquals(
      Seq(Coord(0, 0), Coord(8, 0), Coord(8, 5), Coord(4, 5), Coord(4, 3)),
      Wires.toCoordinates(Seq(Direction.Right(8), Direction.Up(5), Direction.Left(4), Direction.Down(2))))
  }

  @Test def nearestCrossing1(): Unit = {
    assertEquals(6, Wires.nearestCrossing("R8,U5,L5,D3", "U7,R6,D4,L4"))
  }
  @Test def nearestCrossing2(): Unit = {
    assertEquals(159, Wires.nearestCrossing("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"))
  }
  @Test def nearestCrossing3(): Unit = {
    assertEquals(135, Wires.nearestCrossing("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51","U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"))
  }
  @Test def fewestSteps1(): Unit = {
    assertEquals(30, Wires.fewestSteps("R8,U5,L5,D3", "U7,R6,D4,L4"))
  }
  @Test def fewestSteps2(): Unit = {
    assertEquals(610, Wires.fewestSteps("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"))
  }
  @Test def fewestSteps3(): Unit = {
    assertEquals(410, Wires.fewestSteps("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51","U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"))
  }
}

def assertThrows[T](lambda: () => Unit) = {
  try {
    lambda()
    assert(false, "failed to throw")
  } catch {
    case ex => true
  }
}

class TestWireSegment {
  @Test def intersectsVertically1(): Unit = {
    assertEquals(
      Some(4),
      WireSegment(0, 0, 8, 0).intersectsVertically(WireSegment(4, -1, 4, 1)))

  }
  @Test def intersectsVertically2(): Unit = {
    assertThrows(
      () => WireSegment(4, 0, 4, 4).intersectsVertically(WireSegment(4, 1, 8, 1)))
  }
  @Test def intersectsVertically3(): Unit = {
    assertEquals(
      None,
      WireSegment(0, 0, 8, 0).intersectsVertically(WireSegment(9, 1, 10, 1)))
  }
  @Test def intersectsHorizontally1(): Unit = {
    assertEquals(
      None,
      WireSegment(0, 0, 8, 0).intersectsHorizontally(WireSegment(0, 1, 8, 1)))

  }
  @Test def intersectsHorizontally2(): Unit = {
    assertThrows(
      () => WireSegment(0, 0, 0, 8).intersectsHorizontally(WireSegment(4, 1, 8, 7)))

  }
  @Test def intersectsHorizontally3(): Unit = {
    assertEquals(
      Some(1),
      WireSegment(0, 0, 0, 8).intersectsHorizontally(WireSegment(9, 1, 10, 1)))
  }
  @Test def parallel(): Unit = {
    assertEquals(
      None,
      WireSegment(0, 0, 8, 0).intersection(WireSegment(0, 1, 8, 1)))
  }
  @Test def colocated(): Unit = {
    assertThrows(
      () => WireSegment(5, 0, 8, 0).intersection(WireSegment(5, 0, 8, 0)))
  }
  @Test def overlapping(): Unit = {
    assertThrows(
      () => WireSegment(5, 0, 8, 0).intersection(WireSegment(3, 0, 9, 0)))
  }
  @Test def crossing(): Unit = {
    assertEquals(
      Some(Coord(6, 5)),
      WireSegment(4, 5, 8, 5).intersection(WireSegment(6, 1, 6, 9)))
  }
  @Test def notCrossingT(): Unit = {
    assertEquals(
      None,
      WireSegment(4, 5, 8, 5).intersection(WireSegment(6, 7, 6, 9)))
  }
}

class TestCoord {
  @Test def moveRight(): Unit = {
    assertEquals(Coord(0, 0).move(Direction.Right(3)), Coord(3, 0))
  }
  @Test def moveLeft(): Unit = {
    assertEquals(Coord(0, 0).move(Direction.Left(3)), Coord(-3, 0))
  }
  @Test def moveUp(): Unit = {
    assertEquals(Coord(0, 0).move(Direction.Up(3)), Coord(0, 3))
  }
  @Test def moveDown(): Unit = {
    assertEquals(Coord(0, 0).move(Direction.Down(3)), Coord(0, -3))
  }
}