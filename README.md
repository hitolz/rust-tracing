# tracing
tracing 此次演示使用的还是0.1 版本，0.2 版本不是正式版本。

tracing 并不是一个日志库，而是一个分布式跟踪的 SDK，用来采集监控数据的。但是它也支持 log 门面库 API，所以可以当做日志库来使用。

##  span、event、Collector
tracing 中最重要的三个核心概念 span、event、Collector收集器。

### span
span 最大的意义在于它可以记录一个过程，也就是在一段时间内发生的事件流。有开始和结束。

```rust
fn main() {
    init_log(); // 使用 tracing_subscriber 初始化 Collector
    let span = span!(Level::INFO, "span_for_test");
    let _enter = span.enter(); // enter 后进入该 span 的上下文
    info!("hello from span ")
} // 离开作用域后，_enter 被 drop，对应的 span 在此结束
```

输出内容：`2023-11-22 10:40:41.669  INFO span_for_test: rust_tracing: hello from span`

使用`#[instrument]` 创建 span。
当使用了`#[instrument]` ，tracing 会为以后函数主动创立 span ，该 span 名与函数雷同，并且整个函数都在该 span 的上下文内。
```rust
#[instrument]
fn expensive_work(secs: u64) {
    info!("doing expensive work");
    sleep(Duration::from_secs(secs));
    info!("done with expensive work");
}

fn main() {
    init_log(); // 使用 tracing_subscriber 初始化 Collector
    let span = span!(Level::INFO, "span_for_test");
    let _enter = span.enter(); // enter 后进入该 span 的上下文
    info!("hello from span ");
    expensive_work(2);
} // 离开作用域后，_enter 被 drop，对应的 span 在此结束

```
输出内容为
```log
2023-11-22 10:48:13.086  INFO span_for_test: rust_tracing: hello from span     
2023-11-22 10:48:13.086  INFO span_for_test:expensive_work{secs=2}: rust_tracing: doing expensive work    
2023-11-22 10:48:15.091  INFO span_for_test:expensive_work{secs=2}: rust_tracing: done with expensive work 
```

### event
event 代表了某个时间点发生的事件，跟日志类似，不同的是 event 可以产生在 span 的上下文中。


```rust
fn main() {
    init_log(); // 使用 tracing_subscriber 初始化 Collector
    let span = span!(Level::INFO, "span_for_test");
    let _enter = span.enter(); 
    info!("hello from span ");
    event!(Level::INFO, "event hello from span ");
    expensive_work(2);
} 

--- 输出内容与 info 一样。
2023-11-22 10:49:47.585  INFO span_for_test: rust_tracing: hello from span     
2023-11-22 10:49:47.585  INFO span_for_test: rust_tracing: event hello from span 
```


### Collector
当 span 或 event 发生时，会被实现了 Collect 特征的收集器所记录或聚合，这个过程是通过通知的方式实现的：当 event 发生或者 span 开始/结束时，会调用 Collect 特征的相应方法通知 Collector。

Collector 会将 span 和 event 以一定的格式输出到指定的地方，比如：stdout、stderr、文件、网络等。

tracing-subscriber 提供了 Collector，可以方便的输出事件信息。