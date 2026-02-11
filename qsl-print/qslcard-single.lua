local function generate(args, records)
    return {
        array_value = list({ 1 }),
        map_value = map({ 1 }),
        heuristic1 = { 1, 2 },
        heuristic2 = { a = 1, b = 2 },
    }
end

return {
    generate = generate,
}
