local qso_numbers = {}

local function get_multiplier(number)
    if qso_numbers[number] then
        return nil
    else
        qso_numbers[number] = true
        return number
    end
end

local function initialize(parameter)
    return {
        datetime_offset = "09:00",
    }
end

local function qso_metadata(record)
    return {
        id = string.format("%s-%s-%s", record.band, record.mode, record.call),
        group = record.band,
    }
end

local function process_qso(record)
    local pref = string.sub(record.rx_number, 1, 2)

    return {
        multiplier = get_multiplier(record.rx_number),
        point = pref == ("13" and 2 or 1) + (record.mode == "CW" and 1 or 0),
    }
end

local function summarize(groups)
    return {
        total = 100000,
    }
end

return {
    initialize = initialize,
    qso_metadata = qso_metadata,
    process_qso = process_qso,
    summarize = summarize,
}
