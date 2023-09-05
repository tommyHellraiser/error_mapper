//  TODO: Check if I need to add support for std::io errors
//  TODO: add some sort of logger to log any TheError error into a .log file with a macro, and add a message if the user wants to
//  TODO: add support for the following crates: serde, serde_json, clap, syn, rand, regex, base64, libc, onecell, futures
//  TODO: add logger and determine the log file format to use. The order best one might be:
//   date - time - file - error
/*
For the logger, it'd be cool if by sending parameters you could determine the log level, and log to
different files depending on if it's FATAL, ERROR, WARNING, INFORMATION, DEBUG, TRACE
By default the logger will log in INFORMATION. Enum already created in the the_logger file
*/

//  TODO: add get_line_info! macro
//  TODO: add customizable debug printer macro