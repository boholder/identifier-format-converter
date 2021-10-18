/// 我想象中的成品：
/// naming
///
/// TODO 没有参数，输出简短帮助信息
/// (介绍，USAGE:，OPTIONS:，提示输入--help)
///
/// TODO -h --help 忽略其他参数，展示长帮助信息
/// （介绍，usage，options,常用的example，doc链接,提issue链接）
///
/// TODO README里写一些复杂的example和命令行组合使用用例。
///
/// --------------------------
///
/// 行为：
/// 对输入字符流中所有识别为有效可转换的字符串都做转换。
/// 每个匹配生成一行结果。若出现转换失败，忽略并继续下一个，
/// 不把这个列入结果，返回状态非0，输出错误信息。
///
/// 这个工具要和xargs结合使用。
///
/// > naming camelCase
/// > camelCase CAMEL_CASE camel_case camel-case camelCase CamelCase
/// 每行结果首先输出匹配到的输入，再按顺序输出转换后的词，以空格做分离。
///
/// 最后一个参数作为输入内容
/// TODO -- 之后的内容都将被作为输入（不再解析参数）
/// TODO 怎么判断最后参数一个是文件名？
///
/// TODO -s --strict 如果输入中存在转换失败，
/// 中止，不输出stdin，返回状态非0，输出错误信息。
///
/// TODO -f --filter <格式>（-f"Sskcp"）
/// 在能识别成有效字符串的前提下，只处理指定格式的字符串
///
/// screaming-snake, snake, kebab, camel, pascal
/// S, s, k, c, p
///
/// TODO -o --output <格式> 输出指定的变换格式，默认输出所有5种格式
///
/// TODO -e --eof <flag> 学xargs，读到终止符应该停止。
///
/// TODO --mix （输出样式变更）输出这些格式合在一起的OR形式正则表达式。
///
/// TODO --json （输出样式变更）输出json格式的结构化信息
/// {"origin":"...","regex":"OR组成的正则"，"screaming_snake":"..."}
///
/// 一次调用命令只能建立一种转换映射：
/// A,B格式的字符串，转换成C格式。
///