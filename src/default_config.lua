local posix = require "posix"
local unistd = require "posix.unistd"
local stdlib = require "posix.stdlib"

local function handleErrors(f, ...)
        local val, err = f(...)
        if val == nil then
                io.stderr:write(err .. "\n")
                io.stderr:flush()
        else
                return val
        end
end

function _G.cd(path)
        return handleErrors(unistd.chdir, path)
end

function _G.cmd(...)
        local args = { ... }
        local cmd_args
        if type(args[1]) == "table" then
                cmd_args = args[1]
        else
                cmd_args = args
        end
        return handleErrors(posix.spawn, cmd_args)
end

_G.env = setmetatable({}, {
        __index = function(_, key)
                return stdlib.getenv(key)
        end,
        __newindex = function(_, key, value)
                handleErrors(stdlib.setenv, key, value)
        end
})

local conf = {}

function conf.prompt()
        return unistd.getcwd() .. ": "
end

return conf
