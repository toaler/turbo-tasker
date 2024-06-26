import React, {useEffect, useState} from 'react';
import logger from '../logger';
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import {v4 as uuidv4} from "uuid";
import DeleteIcon from '@mui/icons-material/Delete';
import CommitIcon from '@mui/icons-material/Commit';
import PendingIcon from '@mui/icons-material/Pending';
import AddTaskIcon from '@mui/icons-material/AddTask';
import './StagingTab.css';

const StagingTab = ({ reset, actions, setActions }) => {
    const [isPressed, setIsPressed] = useState(false);

    useEffect(() => {
        if (reset) {
            setActions([]);  // Clears the table, excluding the header
        }
    }, [reset]);

    const handleDelete = async (indexToDelete) => {
        setActions(currentActions => {
            return currentActions.filter((_, index) => index !== indexToDelete);
        });
    };

    const totalBytes = actions.reduce((acc, action) => acc + action.bytes, 0);

    const handleCommit = async () => {
        setIsPressed(!isPressed);
        logger.debug('Commit button clicked!');
        // Filter to get only actions with status 'pending'
        const actionsToCommit = actions.filter(action => action.status === 'pending');

        if (actionsToCommit.length === 0) {
            logger.info("No pending actions to commit.");
            return; // Exit if there are no pending actions
        }

        try {
            const result = await commit(actionsToCommit);
            if (result !== undefined) {
                logger.info("Commit successful, result:", result.toLocaleString());
                // Update status to 'success' for all committed actions
                setActions(currentActions =>
                    currentActions.map(action =>
                        action.status === 'pending' ? { ...action, status: 'success' } : action
                    )
                );
            } else {
                logger.info("Commit successful, but no data returned");
                setActions(currentActions =>
                    currentActions.map(action =>
                        action.status === 'pending' ? { ...action, status: 'success' } : action
                    )
                );
            }
        } catch (error) {
            logger.error("Commit failed with error:", error);
            // Update status to 'failure' for all actions attempted to be committed
            setActions(currentActions =>
                currentActions.map(action =>
                    action.status === 'pending' ? { ...action, status: 'failure' } : action
                )
            );
        }
    };

    async function commit(actionsToSend) {
        try {
            const uid = uuidv4();
            logger.info(`[${uid}] Rust call commit start`);
            let return_value = await invoke('commit', { uid: uid, actions: actionsToSend });
            logger.info(`[${uid}] Rust call commit finished`);
            return return_value;
        } catch (error) {
            logger.error(`Exception occurred during commit`, error);
            throw error;
        }
    };

    const handleCommitEvent = (event) => {
        try {
            const { path } = JSON.parse(event.payload); // Assuming path is directly available in the event payload
            setActions(currentActions => currentActions.map(action =>
                action.path === path ? { ...action, icon: <AddTaskIcon /> } : action
            ));
        } catch (e) {
            logger.error(`Error JSON encoded event ${event}`, e);
        }
    };

    useEffect(() => {
        const unsubscribe = listen("commit-event", handleCommitEvent);

        return () => {
            unsubscribe.then((unsub) => unsub());
        };
    }, []);

    return (
        <div className="staging-tab-container">
            <div className="staging-header-container">
                <div className="flex-container">
                    <div className="flex-row">
                        <div className="flex-item">Bytes in scope</div>
                        <div className="flex-item">{totalBytes.toLocaleString("en-US")}</div>
                    </div>
                </div>
                <button
                    className={`staging-commit ${isPressed ? 'pressed' : ''}`}
                    onClick={handleCommit}
                    disabled={actions.length === 0}
                >
                    <CommitIcon fontSize="large" style={{fontSize: '60px'}}/>
                </button>
            </div>
            <table>
                <thead>
                <tr>
                    <th style={{textAlign: "center"}}>Status</th>
                    <th style={{textAlign: "center"}}>Remove</th>
                    <th style={{textAlign: "center"}}>Action</th>
                    <th style={{textAlign: "left"}}>Resource</th>
                    <th style={{textAlign: "right"}}>Bytes</th>
                </tr>
                </thead>
                <tbody>
                {actions.map((actionObj, index) => (
                    <tr key={index}>
                        <td style={{textAlign: "center"}}>{actionObj.icon || <PendingIcon/>}</td>
                        <td style={{textAlign: "center"}}>
                        <DeleteIcon style={{padding: 0, textAlign: "center"}} onClick={() => handleDelete(index)} />
                        </td>
                        <td style={{textAlign: "center"}}>{actionObj.action}</td>
                        <td>{actionObj.path}</td>
                        <td style={{textAlign: "right"}}>{actionObj.bytes.toLocaleString("en-US")}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default StagingTab;
