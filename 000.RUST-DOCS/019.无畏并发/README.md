# 无畏并发（fearless concurrency）
|摘要|CN|EN|备注|
|-|-|-|-|
|- 并发编程（Concurrent programming）|- 并发编程（Concurrent programming），代表程序的不同部分相互独立地执行|- Concurrent programming, in which different parts of a program execute independently,|- 如: 当一个个人在任何一个任务完成前同时处理多个任务，这就是 并发<sup>例子背景: 团队分割方法来开发一个软件项目</sup>|
|-|-|-|-|
|- 并行编程（parallel programming）|- 并行编程（parallel programming）代表程序不同部分同时执行|- parallel programming, in which different parts of a program execute at the same time|- 如: 当你同意将一组任务在组员中分配，每一个组员分配一个任务并单独处理它，这就是 并行 <sup>例子背景: 团队分割方法来开发一个软件项目</sup>|
|-|-|-|-|
|-|-|-|-|
|-|-|-|-|


## 并发处理方式
|并发处理方式|说明|备注|
|-|-|-|
|- 消息传递（message passing）|-|- [消息传递（message passing）](./001.使用消息传递在线程间传送数据.md)|
|-|-|-|
|- 共享内存|- 让多个线程访问相同的共享数据|- [共享状态的并发](./002.共享状态的并发.md)|
|-|-|-|