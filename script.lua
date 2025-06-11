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
   if str:sub(-#suffix) == suffix then
       return str:sub(1, -#suffix - 1), true
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
function tableHas(t, elem)
	for i = 1, #t do
		if t[i] == elem then
			return true
		end
	end
	return false
end

local toDelete = {};
function OnBeforeCompilation()
	bprint("OnBeforeCompilation")
	ident()
	local file = vmprotect.core():outputArchitecture()
	
	local mapFunctions = file:mapFunctions()
	local functions = file:functions()

	for i = 1, mapFunctions:count() do
		local fn = mapFunctions:item(i)
		local name, needs_processing = remove_prefix(fn:name(), "vmprotect_");
		
		if needs_processing then
			local name, processing_kind = choose_prefix(name, {"ultra_", "virtualize_", "mutate_", "destroy_"})
			local name, lock_to_key = remove_prefix(name, "lock_")
			name = name:match("^(.-)_[%d]+$")
			bprint("Processing " .. name .. " (" .. fn:address():tostring() .. ")")
			ident()
			
			local addType
			if processing_kind == "ultra_" then
				bprint("Mutating + virtualizing")
				addType = CompilationType.Ultra
			elseif processing_kind == "virtualize_" then
				bprint("Virtualizing")
				addType = CompilationType.Virtualization
			elseif processing_kind == "mutate_" then
				bprint("Mutating")
				addType = CompilationType.Mutation
			elseif processing_kind == "destroy_" then
				bprint("Destroying")
				addType = nil
			end

			local added = file:functions():addByAddress(fn:address(), addType)
			
			if processing_kind == "destroy_" then
				added:destroy()
			end
			
			if lock_to_key then
				if addType ~= CompilationType.Virtualization and addType ~= CompilationType.Ultra then
					error("Lock requires virtualization")
				end
				bprint("And locking by key")
				added:setLockToKey(true)
			end
			
			bprint("And queuing for export removal")
			table.insert(toDelete, fn:address())
			deent()
		end
	end
	deent()
end
function OnBeforeSaveFile()
	bprint("OnBeforeSaveFile")
	ident()
	local file = vmprotect.core():outputArchitecture()
	
	local exports = file:exports()
	for i = exports:count(), 1, -1 do
		local export = exports:item(i)
		local address = export:address();
		
		if tableHas(toDelete, address) then
			bprint("Deleting export " .. export:name() .. " (" .. address:tostring() .. ")")
			exports:delete(i)
		end
	end
	deent()
end
