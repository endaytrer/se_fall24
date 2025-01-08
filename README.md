# 软件工程专题（2024秋）作业 (v2)

## 简介
[游戏网址](https://danielgu.org/sites/sea/)

使用Rust + WebAssembly编写的双用户界面（命令行+Web前端）游戏，完成作业3内容。

**Web界面**：

![image](https://github.com/user-attachments/assets/6d41fd05-bfd9-4871-8cda-af6c19681bc6)

### Web前端玩法介绍
**游戏目标**： 在HP降为0之前，捕获至少Target条马林鱼（正下方显示了进度），并返回**港口**。

点击相邻（鼠标悬停会标记为蓝色高亮）的区块以移动。

你的出生点为**港口**。除了移动外，在港口无法做任何操作；但是，在港口可以免受鲨鱼的靠近和攻击。

菜单左下角的**指南针**，**红色**箭头指向了港口的位置。左边的数字显示了港口的方位角（正北为0°，正东为90°），以及到港口的距离，一格为1海里（NM）。

使用菜单右下角的**双筒望远镜(Binocular)**（点击菜单栏图标，或直接点击小船及所在格）发现2格以内的马林鱼并且标记。之后，所有已标记的马林鱼会显示为**涟漪**：

![ripple](https://github.com/user-attachments/assets/8b14e7da-4927-4747-9bbe-c3a4d34b4e3e)

只有已标记的马林鱼彩才可被捕获。

点击菜单右下角的**渔网(Capture!)**，或者按住Shift, 以切换捕捞 / 非捕捞状态。在捕捞状态时，选择一个相邻的区块，可以以一定概率捕捉到该区块内的马林鱼。若捕捉失败，马林鱼的生命值会降低（受伤）。受伤的马林鱼会吸引附近的鲨鱼。

若你离**鲨鱼**：

![shark](https://github.com/user-attachments/assets/e29c44e8-f105-4cfd-9404-e0fdc1e67350)

足够近，你就会被鲨鱼追击。每次被鲨鱼攻击之后，你的生命值（下方红心）会降低。鲨鱼也会攻击附近的马林鱼，因此你可以利用马林鱼使鲨鱼分心。你可以**点击**任意位置的鲨鱼以攻击之。遭受若干次攻击后，鲨鱼会死亡。


### 命令行：
具体说明见运行初始界面：

![image](https://github.com/user-attachments/assets/9e2c6559-c71c-4078-960d-271c5622657c)
![image](https://github.com/user-attachments/assets/a01ba59b-9e41-4aab-b8a0-11f49a8ef593)

## Quickstart

```bash
cargo run --bin app --features cli
```

## 打包
```bash
(cd www && npm i && npm run build）
```
构建完成的文件在www/dist中。
