# **FAS-RS**

- [简体中文](README.md)
- [![Stars](https://img.shields.io/github/stars/shadow3aaa/fas-rs)](https://github.com/shadow3aaa/fas-rs)
- [![CI Build](https://img.shields.io/github/actions/workflow/status/shadow3aaa/fas-rs/ci.yml)](https://github.com/shadow3aaa/fas- rs/actions)
- [![Release](https://img.shields.io/github/v/release/shadow3aaa/fas-rs)](https://github.com/shadow3aaa/fas-rs/releases/latest)
- [![Download Total](https://img.shields.io/github/downloads/shadow3aaa/fas-rs/total)](https://github.com/shadow3aaa/fas-rs/releases)

## **Introduction**

  > If the picture seen by the naked eye can be directly reflected in the scheduling, that is to say, the scheduler is placed from the perspective of the viewer to determine the performance, can perfect performance control and maximized experience be achieved? `FAS (Frame Aware Scheduling) )` is this scheduling concept, trying to control performance by monitoring screen rendering to minimize overhead while ensuring rendering time.

- ### **What is `fas-rs`?**

  - `fas-rs` is an implementation of `FAS (Frame Aware Scheduling)` running in user mode. Compared with `MI FEAS` in kernel mode, it has the same core idea but has the advantages of almost universal compatibility and flexibility on any device.
  - Compared with other user-mode `FAS` implementations (such as `scene fas`), `fas-rs` uses a more intrusive inline hook method to obtain rendering time, which brings more accurate data and smaller overhead. However, this is essentially an injection and may be misjudged by the anti-cheating system, although I have not encountered it yet.

## **Customization (configuration)**

- ### **Configuration path: `/sdcard/Android/fas-rs/games.toml`**

- ### **Parameter (`config`) description:**

  - **keep_std**

    - Type: `Bool`
    - `true`: Always keep the standard configuration profile when merging configurations, retain the local configuration application list, and other places are the same as false *
    - `false` : see [default behavior of config merge](#config merge)

  - `*` : default configuration

- ### **Game list (`game_list`) description:**

  - **`"package"` = `target_fps`**

    - `package`: string, application package name
    - `target_fps`: an array (such as `[30, 60, 120, 144]`) or a single integer, indicating the target frame rate that the game will render to, `fas-rs` will dynamically match it at runtime

- ### **`powersave` / `balance` / `performance` / `fast` Description:**

  - **mode:**
    - Currently, `fas-rs` does not have an official switching mode manager, but is connected to the configuration interface of [`scene`](https://www.coolapk.com/apk/com.omarea.vtools). If you don’t use scene, the configuration of `balance` will be used by default.
    - If you have some understanding of programming on Linux, you can switch to the corresponding mode by writing any one of the 4 modes to the `/dev/fas_rs/mode` node, and at the same time, reading it can also know the current `fas-rs` mode
  - **Parameter Description :**
    - fas_boost(bool): The purpose of `fas-rs` is to limit power consumption or reduce game frame drops. When true, it is the mode to reduce frame drops.
    - scale(f64): The number of frame drops that `fas-rs` can tolerate
    - jank_scale(f64): `fas-rs` determines the number of dropped frames due to minor lags
    - big_jank_scale(f64): `fas-rs` determines the number of dropped frames due to large lags
    - use_performance_governor(bool): Whether `fas-rs` uses the performance kernel cpufreq policy when working (this configuration is invalid when fas_boost is turned on)

### **`games.toml` configuration standard example:**

```
[config]
keep_std = true

[game_list]
"com.hypergryph.arknights" = [30, 60]
"com.miHoYo.Yuanshen" = [30, 60]
"com.miHoYo.enterprise.NGHSoD" = [30, 60, 90]
"com.miHoYo.hkrpg" = [30, 60]
"com.mojang.minecraftpe" = [60, 120]
"com.netease.party" = [30, 60]
"com.shangyoo.neon" = 60
"com.tencent.tmgp.pubgmhd" = [60, 90, 120]
"com.tencent.tmgp.sgame" = [30, 60, 90, 120]

[powersave]
fas_boost = false
scale=0.5
jank_scale = 3.0
big_jank_scale = 5.0
use_performance_governor = false

[balance]
fas_boost = false
scale=0.25
jank_scale= 3.0
big_jank_scale = 5.0
use_performance_governor = true

[performance]
fas_boost = false
scale=0.2
jank_scale= 1.5
big_jank_scale = 3.0
use_performance_governor = true

[fast]
fas_boost = true
scale=0.1
jank_scale= 1.5
big_jank_scale = 3.0
use_performance_governor = false
```

## **Configuration merge**

- ### `fas-rs` has a built-in configuration merging system to solve the problem of future configuration function changes. It behaves as follows

  - Delete configurations that do not exist in the local configuration and standard configuration
  - Insert the configuration where the local configuration is missing and the standard configuration exists
  - Retain configurations that exist in both standard and local configurations

- ### Notice

  - Implemented using automatic serialization and deserialization, unable to save non-serialization necessary information such as comments
  - The automatic merged configuration during installation will not be applied immediately, otherwise it may affect the operation of the current version. Instead, the local one will be replaced with the new merged configuration during the next restart.

- ### Manual merge

  - The module will be automatically called once every time it is installed.
  - Manual example

    ```bash
    fas-rs merge /path/to/std/profile
    ```

## **Compile**

```bash
#Termux
apt install rust zip ndk* clang binutils-is-llvm shfmt git-lfs

# Ubuntu(NDK is required)
apt install gcc-multilib git-lfs clang
#Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
#Cargo-ndk
cargo install cargo-ndk

#Clone
git clone https://github.com/shadow3aaa/fas-rs
cd fas-rs

#Compile
chmod +x ./make.sh
./make.sh build --release
```

- ## **💩**

I'm here to introduce you the greatest thief @tryigitx !
So, what did he do?

- 1. He kept stealing FAS-RS module from me without any permission, and pretended to be co-developed with me.
In fact, he hasn't developed any kind of project, just because he is not able to.
  From his homepage (<https://linktr.ee/tryigitx>), we can see he is an eXpErT! lmfao🤣🤣
  I just can't imagine how can a real expert do these things. If @tryigitx is a real expert, pLeAsE fOrGiVe Me😭😭

- 2. He also stole other modules, like "Play Integrity Fix".
  He copied it and changed the author to his own, but that's not all.
  Maybe it's some kind of self-deception, he also changed the name to "China Play Integrity Fix".
  He seemed to want to express that this is for China Version ROMs, but everyone can see what he really wanted to do.
  Now that you've all seen these, make your own judgment based on your own values.
