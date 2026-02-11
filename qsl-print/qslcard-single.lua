---@param args table<string,string>
---@param entries QslCardEntry[]
local function generate(args, entries)
    local converted_entries = {}
    for i, e in ipairs(entries) do
        local bureau_call = e.qso.call
        local call = e.qso.call
        local call_suffix_index = call:find("/")
        local via = false
        local datetime = e.qso.datetime
        local timezone = "UTC"

        if e.misc.manager then
            bureau_call = e.misc.manager
            via = true
        elseif call_suffix_index ~= nil then
            bureau_call = bureau_call:sub(1, call_suffix_index - 1)
        end

        if e.qso.mode ~= "FT8" then
            datetime = datetime:to_offset("+09:00")
            timezone = "JST"
        end

        table.insert(converted_entries, {
            via = via,
            bureau_call = bureau_call,
            call = call,
            date = datetime.date_str,
            time = datetime.time_str,
            timezone = timezone,
            report = e.exchange.tx_report,
            freq = e.qso.freq_str,
            mode = e.qso.mode,
        })
    end

    return converted_entries
end

return {
    generate = generate,
}
