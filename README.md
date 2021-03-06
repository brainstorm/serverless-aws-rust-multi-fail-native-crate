## How to reproduce the error

Assuming you have a properly configured [AWS Rust toolchain](https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/) for the occasion:

```
$ npm install serverless
$ npm install serverless-rust
$ git clone https://github.com/brainstorm/serverless-aws-rust-multi-fail-native-crate && cd serverless-aws-rust-multi-fail-native-crate
$ export SLS_DEBUG=*
$ serverless deploy
```

This will compile the lambda functions `hello` and `world` successfully but fail on the `htslib` one, like this:

```
$ serverless deploy
Serverless: Load command config
Serverless: Load command config:credentials
Serverless: Load command create
Serverless: Load command install
Serverless: Load command package
Serverless: Load command deploy
Serverless: Load command deploy:function
Serverless: Load command deploy:list
Serverless: Load command deploy:list:functions
Serverless: Load command invoke
Serverless: Load command invoke:local
Serverless: Load command info
Serverless: Load command logs
Serverless: Load command metrics
Serverless: Load command print
Serverless: Load command remove
Serverless: Load command rollback
Serverless: Load command rollback:function
Serverless: Load command slstats
Serverless: Load command plugin
Serverless: Load command plugin
Serverless: Load command plugin:install
Serverless: Load command plugin
Serverless: Load command plugin:uninstall
Serverless: Load command plugin
Serverless: Load command plugin:list
Serverless: Load command plugin
Serverless: Load command plugin:search
Serverless: Load command config
Serverless: Load command config:credentials
Serverless: Load command rollback
Serverless: Load command rollback:function
Serverless: Invoke deploy
Serverless: Invoke package
Serverless: Invoke aws:common:validate
Serverless: Invoke aws:common:cleanupTempDir
Serverless: Building native Rust hello func...
[0m[0m[1m[32m    Finished[0m release [optimized] target(s) in 0.07s
objcopy: stfZHoqt: debuglink section already exists
  adding: bootstrap (deflated 61%)
Serverless: Building native Rust world func...
[0m[0m[1m[32m    Finished[0m release [optimized] target(s) in 0.08s
objcopy: stNbKyri: debuglink section already exists
  adding: bootstrap (deflated 61%)
Serverless: Building native Rust htslib func...
[0m[0m[1m[32m   Compiling[0m htslib v0.1.0 (/code/htslib)
[0m[1m[38;5;9merror[0m[0m[1m: linking with `cc` failed: exit code: 1[0m
[0m  [0m[0m[1m[38;5;12m|[0m
[0m  [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.0.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.1.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.10.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.11.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.12.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.13.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.14.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.15.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.2.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.3.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.4.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.5.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.6.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.7.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.8.rcgu.o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.htslib.18evr7dw-cgu.9.rcgu.o" "-o" "/code/target/lambda/release/deps/htslib-54828321c8c6404c" "/code/target/lambda/release/deps/htslib-54828321c8c6404c.55qhjnwzuciohral.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/code/target/lambda/release/deps" "-L" "/code/target/lambda/release/build/backtrace-sys-779263468ee9cf16/out" "-L" "/code/target/lambda/release/build/rust-htslib-848f7293ea0f31ac/out" "-L" "/code/target/lambda/release/build/bzip2-sys-ee3a1df763e00b3b/out/lib" "-L" "/usr/lib64" "-L" "/usr/lib64" "-L" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/code/target/lambda/release/deps/librust_htslib-de40bb8c211a0edf.rlib" "/code/target/lambda/release/deps/libitertools-8bdcee680bb7fb9c.rlib" "/code/target/lambda/release/deps/libieee754-31e358538504232c.rlib" "/code/target/lambda/release/deps/liburl-a4a10d0399242bce.rlib" "/code/target/lambda/release/deps/libpercent_encoding-d985814ec59e21ba.rlib" "/code/target/lambda/release/deps/libidna-8324e99afecc7625.rlib" "/code/target/lambda/release/deps/libunicode_normalization-0b43df8177c5285a.rlib" "/code/target/lambda/release/deps/libunicode_bidi-127b6dea613aa070.rlib" "/code/target/lambda/release/deps/libmatches-656cd79ac1f4dbf3.rlib" "/code/target/lambda/release/deps/libbio_types-832930a70811ba8f.rlib" "/code/target/lambda/release/deps/libquick_error-b99603485db0852f.rlib" "/code/target/lambda/release/deps/libregex-f24f68f7337101fc.rlib" "/code/target/lambda/release/deps/libthread_local-17d2289b727a4eaf.rlib" "/code/target/lambda/release/deps/libregex_syntax-247f351685dd2e2d.rlib" "/code/target/lambda/release/deps/libaho_corasick-acfc8bae7e8559a6.rlib" "/code/target/lambda/release/deps/libmemchr-d011937b0e4fb3d0.rlib" "/code/target/lambda/release/deps/liblinear_map-1f78a0ead911ef1d.rlib" "/code/target/lambda/release/deps/libsnafu-46d7ddb048b26e26.rlib" "/code/target/lambda/release/deps/libdoc_comment-c167071d06a64bad.rlib" "/code/target/lambda/release/deps/liblzma_sys-3999bdb5149454e7.rlib" "/code/target/lambda/release/deps/libbzip2_sys-415c2e0bde8bd002.rlib" "/code/target/lambda/release/deps/liblibz_sys-cf28d6800e03b27b.rlib" "/code/target/lambda/release/deps/libnewtype_derive-3845a05c4ec64475.rlib" "/code/target/lambda/release/deps/libcustom_derive-ef5f5fe80dbc7d21.rlib" "/code/target/lambda/release/deps/liblambda_runtime-08e593bd65c621dd.rlib" "/code/target/lambda/release/deps/liblambda_runtime_core-5726a08917f8c538.rlib" "/code/target/lambda/release/deps/liblambda_runtime_client-f9d540065d09a45d.rlib" "/code/target/lambda/release/deps/liblambda_runtime_errors-f43fe81e9900c5f5.rlib" "/code/target/lambda/release/deps/libserde_json-461a6c9ac6cbd3fd.rlib" "/code/target/lambda/release/deps/libryu-4ee08cd2532a545c.rlib" "/code/target/lambda/release/deps/libserde-a82e1a753d08a8d5.rlib" "/code/target/lambda/release/deps/libhyper-85427dbb57669c95.rlib" "/code/target/lambda/release/deps/libwant-267b55072a92d27e.rlib" "/code/target/lambda/release/deps/libtry_lock-c3ab9ae9b8d10427.rlib" "/code/target/lambda/release/deps/libtokio-43f1161b87ecf065.rlib" "/code/target/lambda/release/deps/libtokio_uds-8506441a55229641.rlib" "/code/target/lambda/release/deps/libmio_uds-86219585ee9abfa5.rlib" "/code/target/lambda/release/deps/libtokio_udp-b8bb6e18baebc5e9.rlib" "/code/target/lambda/release/deps/libtokio_timer-6771792f37cd194f.rlib" "/code/target/lambda/release/deps/libtokio_tcp-33309c4e4b011f56.rlib" "/code/target/lambda/release/deps/libtokio_reactor-7fd76ac98134c672.rlib" "/code/target/lambda/release/deps/libtokio_sync-9d9e71e79b33b64e.rlib" "/code/target/lambda/release/deps/libparking_lot-83e06c11d436b3e7.rlib" "/code/target/lambda/release/deps/libparking_lot_core-0efbb5c6d85ed16f.rlib" "/code/target/lambda/release/deps/libsmallvec-5b05b57becb06706.rlib" "/code/target/lambda/release/deps/liblock_api-c58afde6727867d0.rlib" "/code/target/lambda/release/deps/libowning_ref-12e15978c4ae3c0e.rlib" "/code/target/lambda/release/deps/libstable_deref_trait-59da54a00e5a11ff.rlib" "/code/target/lambda/release/deps/libtokio_fs-bcdc48a51a1b4c94.rlib" "/code/target/lambda/release/deps/libtokio_threadpool-36feed7fc61e2393.rlib" "/code/target/lambda/release/deps/librand-d1cecfb285d5ad80.rlib" "/code/target/lambda/release/deps/librand_xorshift-971113599035a4f8.rlib" "/code/target/lambda/release/deps/librand_pcg-a83aa77287c0fdf2.rlib" "/code/target/lambda/release/deps/librand_hc-2a2dd98b26864e90.rlib" "/code/target/lambda/release/deps/librand_chacha-080869d01882a36f.rlib" "/code/target/lambda/release/deps/librand_isaac-888bdaae0941b9b9.rlib" "/code/target/lambda/release/deps/librand_core-56b6b10882a2e440.rlib" "/code/target/lambda/release/deps/librand_os-54182a7e929439c9.rlib" "/code/target/lambda/release/deps/librand_jitter-d764b0bbad4f04e0.rlib" "/code/target/lambda/release/deps/librand_core-8953e16904ba94bd.rlib" "/code/target/lambda/release/deps/libcrossbeam_queue-c425fc835ce55985.rlib" "/code/target/lambda/release/deps/libcrossbeam_deque-46a3fa3369ee72ac.rlib" "/code/target/lambda/release/deps/libcrossbeam_epoch-afc50c84258e89e7.rlib" "/code/target/lambda/release/deps/libscopeguard-fd5f4664668dd346.rlib" "/code/target/lambda/release/deps/libmemoffset-0d5b2db1b25d9c88.rlib" "/code/target/lambda/release/deps/libarrayvec-eb97ef6d9047ce1d.rlib" "/code/target/lambda/release/deps/libnodrop-bdc84fbe608ce13e.rlib" "/code/target/lambda/release/deps/libtokio_current_thread-1d397d37dbaa4841.rlib" "/code/target/lambda/release/deps/libtokio_executor-0910884687d25869.rlib" "/code/target/lambda/release/deps/libcrossbeam_utils-998b26ea9be0d66d.rlib" "/code/target/lambda/release/deps/liblazy_static-9927bdb5a51d8866.rlib" "/code/target/lambda/release/deps/libtokio_codec-0300b0ffc9e4fb06.rlib" "/code/target/lambda/release/deps/libmio-e64b66169d33f9c1.rlib" "/code/target/lambda/release/deps/libnet2-142f19cc6e7e7e17.rlib" "/code/target/lambda/release/deps/libhttparse-b16dabd259454e4e.rlib" "/code/target/lambda/release/deps/libhttp_body-82913d08df99a371.rlib" "/code/target/lambda/release/deps/libtokio_buf-9ba6462555ef546a.rlib" "/code/target/lambda/release/deps/libh2-080834fe6ff5e272.rlib" "/code/target/lambda/release/deps/libindexmap-72b72d57858d4808.rlib" "/code/target/lambda/release/deps/libstring-1df379016766da52.rlib" "/code/target/lambda/release/deps/libslab-aca7457c7f2adc8c.rlib" "/code/target/lambda/release/deps/libhttp-18b69dbbc862eb9f.rlib" "/code/target/lambda/release/deps/libitoa-8b90c0416aaa1224.rlib" "/code/target/lambda/release/deps/libfnv-e24d8f06a947b9e8.rlib" "/code/target/lambda/release/deps/libtokio_io-dccb3abf642f43c6.rlib" "/code/target/lambda/release/deps/liblog-019071cb8c81e4f0.rlib" "/code/target/lambda/release/deps/libfutures_cpupool-831d1e043c4027d8.rlib" "/code/target/lambda/release/deps/libnum_cpus-e8a80f4367802145.rlib" "/code/target/lambda/release/deps/libfutures-7874ef4b92332821.rlib" "/code/target/lambda/release/deps/libbytes-50d9bdc9b4b451c4.rlib" "/code/target/lambda/release/deps/libeither-b790ab528677bf27.rlib" "/code/target/lambda/release/deps/libiovec-a1028a2bfe268d7f.rlib" "/code/target/lambda/release/deps/libbyteorder-8dbacdfd1cfce70a.rlib" "/code/target/lambda/release/deps/libchrono-63c6b83ca8c4752e.rlib" "/code/target/lambda/release/deps/libnum_integer-66206221af7b46a4.rlib" "/code/target/lambda/release/deps/libnum_traits-acc8e01bb2a7d842.rlib" "/code/target/lambda/release/deps/libtime-6ce88697e34eb71f.rlib" "/code/target/lambda/release/deps/libfailure-eee266e03c50be59.rlib" "/code/target/lambda/release/deps/libbacktrace-8b21969314d448db.rlib" "/code/target/lambda/release/deps/libbacktrace_sys-bb4b7d368a6d499c.rlib" "/code/target/lambda/release/deps/liblibc-c1af9ba80fb01a25.rlib" "/code/target/lambda/release/deps/libcfg_if-2a2fb6f1f35783e3.rlib" "/code/target/lambda/release/deps/librustc_demangle-d99ca56a90172db5.rlib" "-Wl,--start-group" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-fae576517123aa4e.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a72070139220275e.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-093434daf7d99801.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-24daf38551b7a03b.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-36d70d9746402ce9.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-7acfc843240167a8.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-eb2e0f5fe057b8b3.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-75e9ddd83715a368.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-af51e7c6fd7d1248.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-27f2a77b2995d98c.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-ad10152c26711a1e.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-291bd2456cb6c9fe.rlib" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fc6e9071307a3016.rlib" "-Wl,--end-group" "/root/.rustup/toolchains/1.39.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-ebe4001ded7f33e7.rlib" "-Wl,-Bdynamic" "-llzma" "-lz" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"[0m
[0m  [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: /code/target/lambda/release/deps/librust_htslib-de40bb8c211a0edf.rlib(rust_htslib-de40bb8c211a0edf.rust_htslib.9x45zxq7-cgu.11.rcgu.o): In function `rust_htslib::bam::Reader::new::ha6e670d19755b3f5':[0m
[0m          rust_htslib.9x45zxq7-cgu.11:(.text._ZN11rust_htslib3bam6Reader3new17ha6e670d19755b3f5E+0x4c): undefined reference to `sam_hdr_read'[0m
[0m          /code/target/lambda/release/deps/librust_htslib-de40bb8c211a0edf.rlib(rust_htslib-de40bb8c211a0edf.rust_htslib.9x45zxq7-cgu.11.rcgu.o): In function `_$LT$rust_htslib..bam..Reader$u20$as$u20$core..ops..drop..Drop$GT$::drop::hf79152081ffeff3d':[0m
[0m          rust_htslib.9x45zxq7-cgu.11:(.text._ZN66_$LT$rust_htslib..bam..Reader$u20$as$u20$core..ops..drop..Drop$GT$4drop17hf79152081ffeff3dE+0x5): undefined reference to `hts_close'[0m
[0m          /code/target/lambda/release/deps/librust_htslib-de40bb8c211a0edf.rlib(rust_htslib-de40bb8c211a0edf.rust_htslib.9x45zxq7-cgu.11.rcgu.o): In function `rust_htslib::bam::hts_open::h7ce9e273d2825128':[0m
[0m          rust_htslib.9x45zxq7-cgu.11:(.text._ZN11rust_htslib3bam8hts_open17h7ce9e273d2825128E+0xc3): undefined reference to `hts_open'[0m
[0m          /code/target/lambda/release/deps/librust_htslib-de40bb8c211a0edf.rlib(rust_htslib-de40bb8c211a0edf.rust_htslib.9x45zxq7-cgu.11.rcgu.o): In function `_$LT$rust_htslib..bam..HeaderView$u20$as$u20$core..ops..drop..Drop$GT$::drop::he409dea7546339a3':[0m
[0m          rust_htslib.9x45zxq7-cgu.11:(.text._ZN70_$LT$rust_htslib..bam..HeaderView$u20$as$u20$core..ops..drop..Drop$GT$4drop17he409dea7546339a3E+0xb): undefined reference to `bam_hdr_destroy'[0m
[0m          collect2: error: ld returned 1 exit status[0m
[0m          [0m

[0m[1m[38;5;9merror[0m[0m[1m: aborting due to previous error[0m

[0m[0m[1m[31merror[0m[1m:[0m could not compile `htslib`.

To learn more, run the command again with --verbose.
Serverless: Dockerized Rust build encountered an error: undefined 1.
 
  Error --------------------------------------------------
 
 
     For debugging logs, run again after setting the "SLS_DEBUG=*" environment variable.
 
  Stack Trace --------------------------------------------
 
Error
    at functions.forEach.funcName (/home/limsadmin/dev/serverless-aws-rust-multi/node_modules/serverless-rust/index.js:129:15)
    at Array.forEach (native)
    at RustPlugin.build (/home/limsadmin/dev/serverless-aws-rust-multi/node_modules/serverless-rust/index.js:110:22)
    at BbPromise.reduce (/usr/lib/node_modules/serverless/lib/classes/PluginManager.js:448:55)
From previous event:
    at PluginManager.invoke (/usr/lib/node_modules/serverless/lib/classes/PluginManager.js:448:22)
    at PluginManager.spawn (/usr/lib/node_modules/serverless/lib/classes/PluginManager.js:466:17)
    at Deploy.BbPromise.bind.then (/usr/lib/node_modules/serverless/lib/plugins/deploy/deploy.js:122:50)
From previous event:
    at Object.before:deploy:deploy [as hook] (/usr/lib/node_modules/serverless/lib/plugins/deploy/deploy.js:107:10)
    at BbPromise.reduce (/usr/lib/node_modules/serverless/lib/classes/PluginManager.js:448:55)
From previous event:
    at PluginManager.invoke (/usr/lib/node_modules/serverless/lib/classes/PluginManager.js:448:22)
    at PluginManager.run (/usr/lib/node_modules/serverless/lib/classes/PluginManager.js:479:17)
    at variables.populateService.then (/usr/lib/node_modules/serverless/lib/Serverless.js:112:33)
    at runCallback (timers.js:672:20)
    at tryOnImmediate (timers.js:645:5)
    at processImmediate [as _immediateCallback] (timers.js:617:5)
From previous event:
    at Serverless.run (/usr/lib/node_modules/serverless/lib/Serverless.js:99:6)
    at serverless.init.then (/usr/lib/node_modules/serverless/bin/serverless:43:28)
    at /usr/lib/node_modules/serverless/node_modules/graceful-fs/graceful-fs.js:111:16
    at /usr/lib/node_modules/serverless/node_modules/graceful-fs/graceful-fs.js:45:10
    at FSReqWrap.oncomplete (fs.js:123:15)
From previous event:
    at initializeErrorReporter.then (/usr/lib/node_modules/serverless/bin/serverless:43:6)
    at runCallback (timers.js:672:20)
    at tryOnImmediate (timers.js:645:5)
    at processImmediate [as _immediateCallback] (timers.js:617:5)
From previous event:
    at __dirname (/usr/lib/node_modules/serverless/bin/serverless:28:46)
    at Object.<anonymous> (/usr/lib/node_modules/serverless/bin/serverless:65:4)
    at Module._compile (module.js:577:32)
    at Object.Module._extensions..js (module.js:586:10)
    at Module.load (module.js:494:32)
    at tryModuleLoad (module.js:453:12)
    at Function.Module._load (module.js:445:3)
    at Module.runMain (module.js:611:10)
    at run (bootstrap_node.js:394:7)
    at startup (bootstrap_node.js:160:9)
    at bootstrap_node.js:507:3
 
  Get Support --------------------------------------------
     Docs:          docs.serverless.com
     Bugs:          github.com/serverless/serverless/issues
     Issues:        forum.serverless.com
 
  Your Environment Information ---------------------------
     OS:                     linux
     Node Version:           6.16.0
     Serverless Version:     1.46.0
 
```
