local function initialize(parameter)
    return {
        datetime_offset = "09:00",
    }
end

local function process_qso(record)
    print(record.time, record.call, record.tx, record.rx)
    return {
        multiplier = 1,
        point = 1,
    }
end

return {
    initialize = initialize,
    process_qso = process_qso,
}
