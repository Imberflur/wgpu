void switch_default_break(int i)
{
    do {
        break;
    } while(false);
}

void switch_case_break()
{
    switch(0) {
        case 0: {
            break;
        }
        default: {
            break;
        }
    }
    return;
}

void loop_switch_continue(int x)
{
    while(true) {
        bool _continue0 = false;
        switch(x) {
            case 1: {
                _continue0 = true;
                break;
            }
            default: {
                break;
            }
        }
        if (_continue0) {
            continue;
        }
    }
    return;
}

void loop_switch_continue_nesting(int x_1, int y, int z)
{
    while(true) {
        bool _continue1 = false;
        switch(x_1) {
            case 1: {
                _continue1 = true;
                break;
            }
            case 2: {
                switch(y) {
                    case 1: {
                        _continue1 = true;
                        break;
                    }
                    default: {
                        while(true) {
                            bool _continue2 = false;
                            switch(z) {
                                case 1: {
                                    _continue2 = true;
                                    break;
                                }
                                default: {
                                    break;
                                }
                            }
                            if (_continue2) {
                                continue;
                            }
                        }
                        break;
                    }
                }
                if (_continue1) {
                    break;
                }
                break;
            }
            default: {
                break;
            }
        }
        if (_continue1) {
            continue;
        }
    }
    return;
}

[numthreads(1, 1, 1)]
void main(uint3 global_id : SV_DispatchThreadID)
{
    int pos = (int)0;

    DeviceMemoryBarrierWithGroupSync();
    GroupMemoryBarrierWithGroupSync();
    do {
        pos = 1;
    } while(false);
    int _expr4 = pos;
    switch(_expr4) {
        case 1: {
            pos = 0;
            break;
        }
        case 2: {
            pos = 1;
            break;
        }
        case 3:
        case 4: {
            pos = 2;
            break;
        }
        case 5: {
            pos = 3;
            break;
        }
        default:
        case 6: {
            pos = 4;
            break;
        }
    }
    switch(0u) {
        case 0u: {
            break;
        }
        default: {
            break;
        }
    }
    int _expr11 = pos;
    switch(_expr11) {
        case 1: {
            pos = 0;
            break;
        }
        case 2: {
            pos = 1;
            return;
        }
        case 3: {
            pos = 2;
            return;
        }
        case 4: {
            return;
        }
        default: {
            pos = 3;
            return;
        }
    }
}
