import React from 'react'
import './CommandHeader.scss'
import SyncRoundedIcon from '@mui/icons-material/SyncRounded';
import IconButton from '@mui/material/IconButton';
import RestoreRoundedIcon from '@mui/icons-material/RestoreRounded';
import { invoke } from "@tauri-apps/api";
import Button from '@mui/material/Button';

const CommandHeader = () => {

    async function findMonitors() {
        await invoke("find_monitors")
    }
    async function restoreMonitors() {
        const val = await invoke("restore_monitors")
        console.log('sean val ..', val)
    }
    return (
        <div id={'command-header'}>
            {/* <IconButton aria-label="Find Monitors" onClick={findMonitors}>
                <SyncRoundedIcon className={'icon'} />
            </IconButton> */}
            <IconButton title={'Restore DB'} aria-label="Restore DB" onClick={restoreMonitors}>
                <RestoreRoundedIcon className={'icon'} />
            </IconButton>

        </div>
    )
}

export default CommandHeader