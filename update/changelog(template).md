# fas-rs(此处放版本号)

[项目主页](https://github.com/shadow3aaa/fas-rs)

## 更新日志

- commit a
- commit b
- commit c
- ...

## 运行要求

- soc平台无要求
- Android12以上
- zygisk开启并且版本v4以上(magisk v24.0以上并且开启zygisk / ksu + zygisk-next)

## 特殊说明

- 对开启fas的游戏使用shamiko等隐藏可能会导致不生效, 是否生效以/sdcard/Android/fas-rs/fas_log.txt是否有对应游戏记录为准
- 采用zygisk注入劫持libgui获取frametime，存在部分被检测风险
