actor Main
  new create(env: Env) =>
    if env.args.contains("--test", {(l, r) => l == r}) then
      TestMain(env)
    else
      Part1(env)
    end

