-- ==========================
-- Renux OS build.lua (updated)
-- ==========================

local function run(cmd)
    print("\27[34m>> " .. cmd .. "\27[0m")
    local ok, exit_type, code = os.execute(cmd)
    local status = (type(ok) == "number") and ok or (ok and 0) or code
    if status ~= 0 then
      print("\27[31m❌ Failed: " .. cmd .. " (exit " .. tostring(status) .. ")\27[0m")
      os.exit(status)
    end
  end
  
  -- Check if file exists
  local function file_exists(path)
    local f = io.open(path, "r")
    if f then f:close() return true end
    return false
  end
  
  -- Default target architecture
  local ARCH = os.getenv("ARCH") or "x86_64-renux"
  local TARGET = "kernel/arch/x86_64/" .. ARCH .. ".json"
  
  -- Number of parallel jobs
  local function detect_jobs()
    local env = os.getenv("JOBS")
    if env and tonumber(env) then return tonumber(env) end
    local f = io.popen("nproc 2>/dev/null")
    if f then
      local n = tonumber(f:read("*l"))
      f:close()
      if n and n > 0 then return n end
    end
    return 1
  end
  local JOBS = detect_jobs()
  
  -- Workspace name
  local KERNEL = "kernel"
  
  -- Task functions
  local function build_kernel()
    if not file_exists(TARGET) then
      print("\27[31mConfig file not found: " .. TARGET .. "\27[0m")
      print("\27[33mRun 'luajit build.lua help' to see available tasks\27[0m")
      os.exit(1)
    end
    print("\27[33m==> Building kernel (arch="..ARCH..")\27[0m")
    run("RUSTC_WRAPPER=sccache cargo +nightly build -p " .. KERNEL ..
        " --target=" .. TARGET .. " -j " .. JOBS .. " -vv")
    print("\27[32m✅ Kernel build success\27[0m")
  end
  
  local function init_submodules()
    print("\27[33m==> Init submodules\27[0m")
    run("git submodule update --init --recursive")
  end
  
  local function menuconfig()
    print("\27[33m==> Running menuconfig\27[0m")
    run("RUSTC_WRAPPER=sccache cargo +stable run -p menuconfig -j " .. JOBS .. " -vv")
  end
  
  local function run_qemu()
    print("\27[33m==> Launching QEMU\27[0m")
    run("qemu-system-x86_64 -drive format=raw,file=target/"..ARCH.."/debug/bootimage-main.bin")
  end
  
  local function clean_all()
    print("\27[33m==> Cleaning project\27[0m")
    run("cargo clean -vv")
    run("rm -rf target/" .. ARCH)
    run("rm -f renux.iso")
  end
  
  local function show_codename()
    print("\27[36mRenux OS Aurora 0.1.0 (DEV)\27[0m")
  end
  
  local function build_iso()
    print("\27[33m==> Preparing ISO\27[0m")
    run("mkdir -p iso/boot")
    run("cp target/" .. ARCH .. "/debug/bootimage-main.bin iso/boot/")
    run("grub-mkrescue -o renux.iso iso")
    print("\27[32m✅ ISO ready: renux.iso\27[0m")
  end
  
  local function run_iso()
    print("\27[33m==> QEMU ISO boot\27[0m")
    run("qemu-system-x86_64 -cdrom renux.iso")
  end
  
  local function help()
    print("\27[36mRenux OS Build Script\27[0m")
    print("Usage: luajit build.lua [task]\n")
    print("Tasks:")
    print("  init          Initialize git submodules")
    print("  build         Build the kernel")
    print("  menuconfig    Run the menuconfig utility")
    print("  run           Run the Renux OS in QEMU")
    print("  clean         Clean the project and artifacts")
    print("  codename      Show the codename for current release")
    print("  iso           Build the ISO image")
    print("  run_iso       Run the Renux OS ISO in QEMU")
    print("  help          Show this help message")
  end
  
  -- Dispatcher
  local tasks = {
    init       = init_submodules,
    build      = build_kernel,
    menuconfig = menuconfig,
    run        = run_qemu,
    clean      = clean_all,
    codename   = show_codename,
    iso        = build_iso,
    run_iso    = run_iso,
    help       = help,
  }
  
  local t = arg[1]
  if not t then
    help()
    os.exit(0)
  end
  
  if tasks[t] then
    tasks[t]()
  else
    print("\27[31mUnknown task: "..t.."\27[0m")
    help()
    os.exit(1)
  end
    -- End of build.lua
    -- ==========================
    -- Renux OS build.lua (updated)
    -- ==========================
    -- To run this script, use:
    -- luajit build.lua [task]