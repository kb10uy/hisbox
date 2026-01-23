local function initialize(parameter)
    return {
        datetime_offset = "09:00",
    }
end

local function qso_metadata(record)
    return {
        id = record.call,
        group = record.band,
    }
end

local function process_qso(record)
    print(record.time, record.call, record.tx_number, record.rx_number)
    return {
        multiplier = 1,
        point = 1,
    }
end

local function calculate_total(groups)
    return 0
end

return {
    initialize = initialize,
    qso_metadata = qso_metadata,
    process_qso = process_qso,
    calculate_total = calculate_total,
}
