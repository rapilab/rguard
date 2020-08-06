# Learn proguard with Rust



## Document

### [ProGuard](https://github.com/Guardsquare/proguard)

ProGuard 能够对 Java 类中的代码进行压缩（Shrink）,优化（Optimize）,混淆（Obfuscate）,预检（Preveirfy）。

1. 压缩（Shrink）：    在压缩处理这一步中，用于检测和删除没有使用的类，字段，方法和属性。
2. 优化（Optimize）：  在优化处理这一步中，对字节码进行优化，并且移除无用指令。
3. 混淆（Obfuscate）： 在混淆处理这一步中，使用 a,b,c 等无意义的名称，对类，字段和方法进行重命名。
4. 预检（Preveirfy）： 在预检这一步中，主要是在 Java 平台上对处理后的代码进行预检。

### [AndResGuard](https://github.com/shwenzhang/AndResGuard)

> AndResGuard 是一个缩小 APK 大小的工具，原理类似 Java Proguard，但是只针对资源。它会将原本冗长的资源路径变短，例如将 `res/drawable/wechat` 变为 `r/d/a`。

benchmark sample:

 - 31.8M -> 29.6M，少了2.2M

Articles

 - [微信资源混淆AndResGuard源码解析(一)](https://cc1over.github.io/2019/08/08/%E5%BE%AE%E4%BF%A1%E8%B5%84%E6%BA%90%E6%B7%B7%E6%B7%86AndResGuard%E6%BA%90%E7%A0%81%E8%A7%A3%E6%9E%90/)
 - [微信资源混淆AndResGuard源码解析(二)](https://cc1over.github.io/2019/08/09/%E5%BE%AE%E4%BF%A1%E8%B5%84%E6%BA%90%E6%B7%B7%E6%B7%86AndResGuard%E6%BA%90%E7%A0%81%E8%A7%A3%E6%9E%90(%E4%BA%8C)/)
 - [微信资源混淆AndResGuard源码解析(三)](https://cc1over.github.io/2019/08/10/%E5%BE%AE%E4%BF%A1%E8%B5%84%E6%BA%90%E6%B7%B7%E6%B7%86AndResGuard%E6%BA%90%E7%A0%81%E8%A7%A3%E6%9E%90(%E4%B8%89)/)

### 多渠道打包

 - [VasDolly]
 - [Walle]

### AAR ProGuard

### 其它

 - [pyretrace](https://github.com/EverythingMe/pyretrace) A python reimplementation on Proguard's Retrace, with a deobfuscation API for python.
 - [ResProguard](https://github.com/JohnnyYin/ResProguard) C++ 2015 年


