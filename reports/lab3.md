# Lab3
## 实现思路
按照要求，我需要统计不同的 syscall_id 的调用次数，因此需要在`TaskControlBlock`中增加一个`syscall_count`数组，数组的大小为 5，每个元素对应一个系统调用的调用次数。
在 TaskControlBlock 中增加 syscall_count，的好处在于，我不需要修改 TaskContext 中的内容，因为它只负责保存系统调用的上下文，减少它的内容，可以降低保存和恢复的开销。
在 syscall 方法中，在调用对应的 syscall handler 之前，我进行对应的 count + 1 操作。

