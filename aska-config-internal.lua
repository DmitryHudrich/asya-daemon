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
        accepted_users = {
            "ohhh_i_l1ke_u"
        },
    },
    ai = {
        recognize_method = "Groq",
    }
}

return config

