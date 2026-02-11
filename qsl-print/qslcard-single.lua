---@param args table<string,string>
---@param entries QslCardEntry[]
local function generate(args, entries)
    local converted_entries = {}
    for i, e in ipairs(entries) do
        local datetime = e.qso.datetime
        if e.qso.mode ~= "FT8" then
            datetime = datetime:to_offset("+09:00")
        end

        table.insert(converted_entries, {
            date = datetime.date_str,
            time = datetime.time_str,
            mode = e.qso.mode,
            call = e.qso.call,
        })
    end

    return converted_entries
end

return {
    generate = generate,
}
