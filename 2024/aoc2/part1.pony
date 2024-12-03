use "files"
use "collections"

actor Part1
  new create(env: Env) =>
    // Öppna filen för läsning
    try
      let file = File.open(FilePath.create(FileAuth(env.root), env.args(1)?))

      if file.errno() is FileOK then
        // Iterera över varje rad i filen
        var line: String val = ""
        var unsafe: I16 = 0
        for l in file.lines() do
          line = consume l
          // Bearbeta varje rad
          env.out.print(line)
          var sum:I16 = 0
          let lst = List[String].from(line.split_by(" "))
          let nums = lst.map[I16]({(s:String) => try s.i16()? else 0 end })
          var last = nums.shift()?
          var isNeg: (Bool|None) = None
          unsafe = unsafe +
          for i in nums.values() do
            var diff = last - i
            let sameSign = match (diff < 0, isNeg)
            // just memorize the sign and call them same
            | (let b: Bool, None) => isNeg = b; true
            | (let b: Bool, false) => not b
            | (let b: Bool, true) => b
            else true
            end
            last = i
            // env.out.print(diff.string() + isNeg.string() + sameSign.string())
            if (not sameSign) or (diff.abs() < 1) or (diff.abs() > 3) then
              break 0          
            else 
              1
            end
          else 0
          end
          env.out.print(unsafe.string())
        end
      else
          env.out.print("nope")
      end
    end