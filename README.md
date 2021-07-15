# 📊 gmsv_serverstat

Simple serverside binary module which can expose information about system resource usage to Lua.

## Installation

Download the relevant module for your server's operating system and platform/Gmod branch from the [releases section](https://github.com/WilliamVenner/gmsv_serverstat/releases).

Drop the module into `garrysmod/lua/bin/` in your server's files. If the `bin` folder doesn't exist, create it.

If you're not sure on what operating system/platform your server is running, run this in your server's console:

```lua
lua_run print((system.IsWindows()and"Windows"or system.IsLinux()and"Linux"or"Unsupported").." "..(jit.arch=="x64"and"x86-64"or"x86"))
```

## Usage

### Blocking Functions

Some of these functions may block the main thread whilst acquiring information about the system & process. Make sure to call these functions sparingly.

Some functions will only block when they are called for the first time.

Some functions may also return default values (such as 0) when called for the first time.

```lua
if not serverstat then require("serverstat") end

-- Gets SRCDS' CPU usage
-- [float] 0..1
serverstat.ProcessCPUUsage()

-- Gets SRCDS' memory usage in MiB
-- [float] MiB
serverstat.ProcessMemoryUsage()

-- Gets the system's total CPU usage
-- [float] 0..1
serverstat.SystemCPUUsage()

-- Gets the system's current memory usage in MiB
-- [float] MiB
serverstat.SystemMemoryUsage()

-- Gets the system's total memory installed in MiB
-- [float] MiB
serverstat.SystemTotalMemory()

-- Gets the system's available memory in MiB
-- [float] MiB
serverstat.SystemAvailableMemory()

-- Gets the system's number of physical CPUs (cores)
-- [integer]
serverstat.PhysicalCPUs()

-- Gets the system's number of logical CPUs
-- Roughly equates to physical cores (CPUs) × threads per core
-- [integer]
serverstat.LogicalCPUs()

-- Fetches all resource usage information about the system and process as a table.
-- The table is keyed by the above function names and their respective return data as the value.
-- [table]
serverstat.All()

-- Fetches all SYSTEM resource usage information about the system and process as a table.
-- The table is keyed by the above function names and their respective return data as the value.
-- [table]
serverstat.AllSystem()

-- Fetches all SRCDS resource usage information about the system and process as a table.
-- The table is keyed by the above function names and their respective return data as the value.
-- [table]
serverstat.AllProcess()
```

### Asynchronous Functions

Each blocking function has an asynchronous equivalent in the `serverstat.async` table which takes a single `function` callback argument.

Using the asynchronous functions will acquire the requested information on a separate thread.

The thread goes to sleep when unused and uses no system resources until needed again.

### Realtime Functions

Additionally, serverstat provides a "realtime" data API.

This is to discourage multiple script authors from making what is essentially the same thing; an autorefreshing timer for the data this module provides.

These functions will return data that is updated roughly every 250ms. They are synchronous; updating the data is automatically done in a timer the module creates for you.

This timer will only be created if you start using these functions. It will persist until the server shuts down.

**The realtime data API is deliberately missing functions such as `LogicalCPUs` and `PhysicalCPUs`, because these are constant values.**

```lua
-- You don't need to call these functions; they are provided for convenience if you want to control the realtime updater timer yourself.
serverstat.realtime.Start()
serverstat.realtime.Stop()
serverstat.realtime.SetInterval(seconds)

serverstat.realtime.ProcessCPUUsage()
serverstat.realtime.ProcessMemoryUsage()

serverstat.realtime.SystemCPUUsage()
serverstat.realtime.SystemMemoryUsage()
serverstat.realtime.SystemAvailableMemory()

-- These functions return a reference to the table that is shared
-- with other addons. If you are going to be mutating this table,
-- please use the Copy functions below instead.
serverstat.realtime.All() -- { System = serverstat.realtime.AllSystem(), Process = serverstat.realtime.AllProcess() }
serverstat.realtime.AllSystem() -- { CPUUsage = [float], MemoryUsage = [float] MIB, AvailableMemory = [float] MiB }
serverstat.realtime.AllProcess() -- { CPUUsage = [float], MemoryUsage = [float] MIB }

-- These functions return a copy of the table that is shared
-- with other addons. If you aren't going to be mutating the table,
-- you should just use the above functions instead.
serverstat.realtime.AllCopy()
serverstat.realtime.AllSystemCopy()
serverstat.realtime.AllProcessCopy()
```
