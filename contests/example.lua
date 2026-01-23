local jarl = require("jarl");

local function initialize(parameter)
    print(jarl.example())
    return {
        datetime_offset = "09:00",
    }
end

local function qso_key(record)
    return {
        id = record.call,
        group = record.band,
    }
end

local function process_qso(record)
    print(record.time, record.call, record.tx_number, record.rx_number)
    return {
        multiplier = record.call,
        point = 1,
    }
end

local function summarize(groups)
    return {
        total = 100000,
    }
end

return {
    initialize = initialize,
    qso_key = qso_key,
    process_qso = process_qso,
    summarize = summarize,
}
