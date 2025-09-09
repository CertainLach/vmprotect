local _ident = 0
function ident()
    _ident = _ident + 1
end

function deent()
    _ident = _ident - 1
end

function bprint(smth)
    local buf = ""
    for i = 1, _ident do
        buf = buf .. ".      "
    end
    buf = buf .. smth
    print(buf)
end

function remove_prefix(str, prefix)
    if str:sub(1, #prefix) == prefix then
        return str:sub(#prefix + 1), true
    end
    return str, false
end

function remove_suffix(str, suffix)
    if str:sub(- #suffix) == suffix then
        return str:sub(1, - #suffix - 1), true
    end
    return str, false
end

function choose_prefix(str, prefixes)
    for _, prefix in ipairs(prefixes) do
        if str:sub(1, #prefix) == prefix then
            return str:sub(#prefix + 1), prefix
        end
    end
    error("Unexpected function name")
end

-- FIXME: Use this code if you have older vmprotect version, without rust demangling support
-- local rustc_demangle_lib = vmprotect.openLib(
--     "/home/unq/build/rustc-demangle/target/release/librustc_demangle.so")
-- if not rustc_demangle_lib then
--     error("failed to open librustc_demangle")
-- end
-- local rustc_demangle = rustc_demangle_lib:getFunction("rustc_demangle", {ret = "int", "string", "pointer", "size_t", abi = "cdecl"})
--
-- local c_lib = vmprotect.openLib("/nix/store/maxa3xhmxggrc5v2vc0c3pjb79hjlkp9-glibc-2.40-66/lib/libc.so.6")
-- if not c_lib then
--     error("failed to open libc")
-- end
-- local malloc = c_lib:getFunction("malloc", {ret = "pointer", abi = "cdecl", "size_t"})
-- local memset = c_lib:getFunction("memset", {ret = "pointer", "pointer", "int", "size_t", abi = "cdecl"})
-- local free = c_lib:getFunction("malloc", {ret = "void", "pointer", abi = "cdecl"})
-- local strstr = c_lib:getFunction("strstr", {ret = "string", "pointer", "string", abi = "cdecl"})

-- function demangle(identifier)
--     bprint("malloc")
--     local demangle_buf = malloc(2048)
--     bprint("malloc done")
--     if demangle_buf == nil then
--         error("out of memory?")
--     end
--     bprint("demangle")
--     local res_code = rustc_demangle(identifier, demangle_buf, 4096)
--     if res_code == 0 then
--         free(demangle_buf)
--         return nil
--     end
--     bprint("strstr")
--     local luaStr = strstr(demangle_buf, "")
--     bprint("free")
--     free(demangle_buf)
--
--     return luaStr
-- end

-- if demangle("_RNvCskwGfYPst2Cb_3foo16example_function") ~= "foo::example_function" then
--     error("demangler is not working properly")
-- end

function OnBeforeCompilation()
    bprint("OnBeforeCompilation")
    ident()
    local file = vmprotect.core():outputArchitecture()
    -- explore(file)
    -- explore(file:symbols())

    local mapFunctions = file:mapFunctions()

    for i = 1, mapFunctions:count() do
        local fn = mapFunctions:item(i)
        if fn:type() ~= ObjectType.Code then
            goto continue
        end

        local name = fn:name()

        local protinfo = name:match("VMPROTECT_MARKER_(.-)_END")
        if protinfo == nil then
            goto continue
        end

        -- print("To demangle: " .. name)
        -- if name:sub(1, 2) == "_R" then
        --     name = demangle(name)
        --     if name == nil then
        --         error("Failed to demangle " .. fn:name())
        --     end
        -- end
        -- print("Demangled")

        -- local protinfo;
        -- name, protinfo = name:match("^(.+)::vmprotect_(.+)::h" .. string.rep("[0-9]", 16) .. "$")
        -- if protinfo == nil then
        --     bprint("Failed to parse marker function: " .. fn:name())
        --     goto continue
        -- end

        local processing_kind
        local lock_to_key
        protinfo, processing_kind = choose_prefix(protinfo, { "ultra", "virtualize", "mutate", "destroy" })
        protinfo, lock_to_key = remove_prefix(protinfo, "_lock")
        bprint("Processing " .. name .. " (" .. fn:address():tostring() .. ")")

        ident()

        local addType
        if processing_kind == "ultra" then
            bprint("Mutating + virtualizing")
            addType = CompilationType.Ultra
        elseif processing_kind == "virtualize" then
            bprint("Virtualizing")
            addType = CompilationType.Virtualization
        elseif processing_kind == "mutate" then
            bprint("Mutating")
            addType = CompilationType.Mutation
        elseif processing_kind == "destroy" then
            bprint("Destroying")
            addType = nil
        end
        local added = file:functions():addByAddress(fn:address(), addType)

        if processing_kind == "destroy" then
            added:destroy()
        end

        if lock_to_key then
            if addType ~= CompilationType.Virtualization and addType ~= CompilationType.Ultra then
                error("Lock requires virtualization")
            end
            bprint("And locking by key")
            added:setLockToKey(true)
        end

        deent()

        ::continue::
    end
    deent()
end
