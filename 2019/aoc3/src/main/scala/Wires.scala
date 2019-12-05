case class Coord(x: Int, y: Int) {
  def move(dir: Direction): Coord = {
    import Direction._
    dir match {
      case Right(v) => Coord(x + v, y)
      case Left(v) => Coord(x - v, y)
      case Up(v) => Coord(x, y + v)
      case Down(v) => Coord(x, y - v)
    }
  }
  def distance() = Math.abs(x)+Math.abs(y)
}

case class WireSegment(start: Coord, end: Coord) {
  def isVertical = start.x == end.x
  def isHorizontal = start.y == end.y
  override def toString(): String = {
    s"WireSegment[(${start.x}, ${start.y})-(${end.x}, ${end.y})]"
  }
  def intersectsVertically(other: WireSegment): Option[Int] = {
    val n = WireSegment.normalized(start, end)
    val on = WireSegment.normalized(other.start, other.end)
    val max = Math.max(n.start.x, on.start.x)
    val min = Math.min(n.end.x, on.end.x)
    if (max <= min) {
      if (max == min) {
        Some(max)
      } else {
        if (isHorizontal && other.isHorizontal && start.y == other.start.y)
          throw new NotImplementedError(s"no lines on top of each other allowed, $this, $other")
        else // parallel, but not overlapping
          None
      }
    }
    else None
  }
  def intersectsHorizontally(other: WireSegment): Option[Int] = {
    val n = WireSegment.normalized(start, end)
    val on = WireSegment.normalized(other.start, other.end)
    val max = Math.max(n.start.y, on.start.y)
    val min = Math.min(n.end.y, on.end.y)
    if (max <= min) {
      if (max == min) {
        Some(max)
      } else {
        if (isVertical && other.isVertical && start.x == other.start.x)
          throw new NotImplementedError(s"no lines on top of each other allowed, $this, $other")
        else // parallel, but not overlapping
          None
      }
    }
    else None
  }
  def intersection(other: WireSegment): Option[Coord] = {
    (intersectsVertically(other), intersectsHorizontally(other)) match {
      case (Some(x), Some(y)) => {
        Some(Coord(x, y))
      }
      case _ => None
    }
  }
}

object WireSegment {
  def normalized(start: Coord, end: Coord): WireSegment = {
    if (start.x == end.x) { // horizontal
      if (start.y > end.y) {
        WireSegment(start.x, end.y, end.x, start.y)
      } else {
        WireSegment(start, end)
      }
    } else if (start.y == end.y) { // vertical
      if (start.x > end.x) {
        WireSegment(end.x, start.y, start.x, end.y)
      } else {
        WireSegment(start, end)
      }
    } else {
      throw new NotImplementedError("no slanting lines allowed")
    }
  }

  def apply(x1: Int, y1: Int, x2: Int, y2: Int): WireSegment = WireSegment(Coord(x1, y1), Coord(x2, y2))
}

case class Wire(segments: Seq[WireSegment])
{
  def stepsTo(coord: Coord) = 0
}

object Wires {
  def parseWire(wire: String): Seq[Direction] = {
    wire.split(",").toSeq.map { word =>
      word.head match {
        case 'R' => Direction.Right(Integer.parseInt(word.substring(1)))
        case 'L' => Direction.Left(Integer.parseInt(word.substring(1)))
        case 'U' => Direction.Up(Integer.parseInt(word.substring(1)))
        case 'D' => Direction.Down(Integer.parseInt(word.substring(1)))
      }
    }
  }
  def toCoordinates(wireDirections: Seq[Direction]): Seq[Coord] = {
    wireDirections.scanLeft(Coord(0, 0))((s, d) => s.move(d))
  }
  def toWire(wireDirections: Seq[Direction]): Wire = {
    val coords = toCoordinates(wireDirections)
    Wire(coords.zip(coords drop 1).map((s, e) => WireSegment(s, e)))
  }
  def crossings(wire1: Wire, wire2: Wire): Seq[Coord] = {
    wire1.segments.flatMap { w1s =>
      wire2.segments.map(w2s => w1s.intersection(w2s))
    }.filter(_.isDefined).map(_.get)
  }

  def nearestCrossing(wire1spec: String, wire2spec: String): Int = {
    val wire1 = toWire(parseWire(wire1spec))
    val wire2 = toWire(parseWire(wire2spec))
    val crossingDistances = crossings(wire1, wire2).map(_.distance()).sorted
    crossingDistances.drop(1).head
  }

  def fewestSteps(wire1spec: String, wire2spec: String): Int = {
    val wire1 = toWire(parseWire(wire1spec))
    val wire2 = toWire(parseWire(wire2spec))
    crossings(wire1, wire2).map { crossing =>
      wire1.stepsTo(crossing) + wire2.stepsTo(crossing)
    }.sorted.drop(1).head
  }
}

enum Direction(val distance: Int) {
  case Right(v: Int) extends Direction(v)
  case Left(v: Int) extends Direction(v)
  case Up(v: Int) extends Direction(v)
  case Down(v: Int) extends Direction(v)
}
