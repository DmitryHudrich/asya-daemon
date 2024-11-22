local config = {
    net = {
        http_port = 3001,
        grpc_port = 50051
    },
    logging = {
        place = false,
        level = "info",
        folder = "logs",
        filescount = 5,
        stdout = true
    },
    telegram = {
        token = "7935159432:AAG5NKw6bzN0tLsFbKORByBXA_kp6Qj-CvI",
    },
}

return config

