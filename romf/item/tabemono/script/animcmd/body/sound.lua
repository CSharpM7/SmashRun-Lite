sound_Lost = function ()
    if sv_animcmd.is_excute() then
        sv_animcmd.PLAY_SE_REMAIN(81997679861)
    end
    return 
end

sound_SetBoundSE = function ()
    --[[
    if sv_animcmd.is_excute() then
        sv_animcmd.IT_SET_SE_arg2(90177080504, 65532141872)
    end
    local f2_local0
    if WorkModule:get_int64() ~= ITEM_VARIATION_TABEMONO_MILK then
        f2_local0 = false
    else
        f2_local0 = true
    end
    if f2_local0 == nil then
        if sv_animcmd.is_excute() then
            WorkModule.set_int64(106287966264, ITEM_TABEMONO_INSTANCE_WORK_INT_EAT_SE_HASH)
        end
    else
        f2_local0
        if WorkModule:get_int64() ~= ITEM_VARIATION_TABEMONO_TEA then
            f2_local0 = false
        else
            f2_local0 = true
        end
        if f2_local0 == nil then
            if sv_animcmd.is_excute() then
                WorkModule.set_int64(106287966264, ITEM_TABEMONO_INSTANCE_WORK_INT_EAT_SE_HASH)
            end
        else
            f2_local0
            if WorkModule:get_int64() ~= ITEM_VARIATION_TABEMONO_COLA then
                f2_local0 = false
            else
                f2_local0 = true
            end
            if f2_local0 == nil then
                if sv_animcmd.is_excute() then
                    WorkModule.set_int64(106287966264, ITEM_TABEMONO_INSTANCE_WORK_INT_EAT_SE_HASH)
                end
            else
                f2_local0
                if WorkModule:get_int64() ~= ITEM_VARIATION_TABEMONO_CORNPOTAGE then
                    f2_local0 = false
                else
                    f2_local0 = true
                end
                if f2_local0 == nil then
                    if sv_animcmd.is_excute() then
                        WorkModule.set_int64(106287966264, ITEM_TABEMONO_INSTANCE_WORK_INT_EAT_SE_HASH)
                    end
                elseif sv_animcmd.is_excute() then
                    WorkModule.set_int64(101315242062, ITEM_TABEMONO_INSTANCE_WORK_INT_EAT_SE_HASH)
                end
            end
        end
    end
    ]]
    return 
end

return 
