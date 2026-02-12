---@meta datetime

---@class DateTime
---@field year integer
---@field month integer
---@field day integer
---@field hour integer
---@field minute integer
---@field second integer
---@field date_str string
---@field time_str string
local DateTime = {}

--- Returns new DateTime that points same moment in UTC.
---@return DateTime dt DateTime in UTC.
function DateTime:to_utc() end

--- Returns new DateTime that points same moment in specified offset.
---@param offset string offset value.
---@return DateTime dt DateTime in specified offset.
function DateTime:to_offset(offset) end

--- Formats into string.
---@param desc string format descriptor written in Rust `time` crate style.
function DateTime:format(desc) end

-- Module

---@class datetime
local datetime = {}

--- Returns current datetime in UTC.
---@return DateTime now
function datetime.now_utc() end

--- Returns current datetime in local offset.
---@return DateTime now
function datetime.now_local() end

--- Parses datetime from RFC3339.
---@param dt string RFC3339 datetime string.
---@return DateTime parsed
function datetime.from_rfc3339(dt) end

--- Constructs datetime from date and time.
---@param d string date string.
---@param t string time string.
---@return DateTime parsed
function datetime.from_parts_utc(d, t) end

--- Constructs datetime from date, time, and offset.
---@param d string date string.
---@param t string time string.
---@param o string offset string.
---@return DateTime parsed
function datetime.from_parts_offset(d, t, o) end

return datetime
