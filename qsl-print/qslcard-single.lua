---@param args table
---@param entries QslCardEntry[]
local function generate(args, entries)
    local converted_entries = {}
    for i, e in ipairs(entries) do
        table.insert(converted_entries, {
            date = e.qso.datetime.date_str,
            time = e.qso.datetime.time_str,
            call = e.qso.call,
        })
    end

    return converted_entries
end

return {
    generate = generate,
}
