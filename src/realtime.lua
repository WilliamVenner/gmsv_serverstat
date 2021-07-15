local serverstat = serverstat
local timer_Create = timer.Create
local table_Copy = table.Copy

local serverstat_realtime_RefreshRefreshAsync = serverstat.realtime.RefreshAsync
serverstat.realtime.RefreshAsync = nil

local realtimeData = serverstat.realtime.Refresh()
serverstat.realtime.Refresh = nil

function serverstat.realtime.ProcessCPUUsage()
	return realtimeData.Process.CPUUsage
end
function serverstat.realtime.ProcessMemoryUsage()
	return realtimeData.Process.MemoryUsage
end
function serverstat.realtime.SystemCPUUsage()
	return realtimeData.System.CPUUsage
end
function serverstat.realtime.SystemMemoryUsage()
	return realtimeData.System.MemoryUsage
end
function serverstat.realtime.SystemAvailableMemory()
	return realtimeData.System.AvailableMemory
end
function serverstat.realtime.All()
	return realtimeData
end
function serverstat.realtime.AllSystem()
	return realtimeData.System
end
function serverstat.realtime.AllProcess()
	return realtimeData.Process
end
function serverstat.realtime.AllCopy()
	return table_Copy(realtimeData)
end
function serverstat.realtime.AllSystemCopy()
	return table_Copy(realtimeData.System)
end
function serverstat.realtime.AllProcessCopy()
	return table_Copy(realtimeData.Process)
end

function serverstat.realtime.Stop()
	timer_Remove("gmsv_serverstat_realtime")
end

local pending = false
local interval = .25
local function refresh()
	if pending then return end
	pending = true
	serverstat_realtime_RefreshRefreshAsync(function(data)
		realtimeData = data
		pending = false
	end)
end
function serverstat.realtime.Start()
	timer_Create("gmsv_serverstat_realtime", interval, 0, refresh)
end
serverstat.realtime.Start()

function serverstat.realtime.SetInterval(newInterval)
	assert(tonumber(newInterval) ~= nil, "Expected a number")
	assert(newInterval >= 0, "Expected a positive number")
	interval = tonumber(newInterval)

	if timer.Exists("gmsv_serverstat_realtime") then
		serverstat.realtime.Start()
	end
end