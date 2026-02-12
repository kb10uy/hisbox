---@meta

---@class QslCardEntry
---@field qso Record
---@field exchange Exchange
---@field misc Misc
local QslCardEntry = {}

---@class Record
---@field datetime DateTime
---@field band string
---@field freq number
---@field freq_str string
---@field mode string
---@field call string
local Record = {}

---@class Exchange
---@field tx_report string|nil
---@field tx_number string|nil
---@field rx_report string|nil
---@field rx_number string|nil
local Exchange = {}

---@class Misc
---@field antenna string|nil
---@field power string|nil
---@field operator string|nil
---@field address string|nil
---@field grid string|nil
---@field manager string|nil
local Misc = {}
