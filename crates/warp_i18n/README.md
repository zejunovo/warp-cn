# Warp 界面国际化

运行时、编译期宏和 Fluent 翻译资源移植自
[Heartcoolman/warp-cn](https://github.com/Heartcoolman/warp-cn)，参考提交为
`a714a1ad833053db6f190f7fff38ecccc9fcd6ae`。

本目录及相邻的 `warp_i18n_macros` 保留项目的 AGPL v3 许可，参见根目录
[LICENSE-AGPL](../../LICENSE-AGPL)。

`bundles/en` 与 `bundles/zh-CN` 分别保存英文和简体中文界面文案。新增或修改
文案时，应同时维护两种语言的键和参数；终端输出、代码、用户输入及服务端内容
不属于这份界面翻译资源。

`I18n::new()` 可创建独立实例，方便测试语言切换与通知行为；应用通过 `init()`
使用进程级实例。`t!()` 验证字面量键，`t_static!()` 仅用于要求静态生命周期的
固定界面标签，避免将不断变化的用户内容永久驻留。
