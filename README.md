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

Some of these functions may block the main thread whilst acquiring information about the system & process. Make sure to call these functions sparingly.

Some functions will only block when they are called for the first time.

Some functions may also return default values (such as 0) when called for the first time.

```lua
serverstat = serverstat or require("serverstat")

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
```
