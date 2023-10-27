import React, { useEffect } from 'react'
import Display from '../Display/Display'
import './DisplayContainer.scss'
import { PropTypes } from 'prop-types'
import { useState } from 'react';

const DisplayContainer = ({ monitors }) => {

    const [positions, setPositions] = useState(Array(monitors.length).fill({ x: 0, y: 0 }));
    const [indexMoving, setIndexMoving] = useState(null);
    const [positionInDisplay, setPositionInDisplay] = useState({ left: 0, top: 0 })
    function startMoveDisplay(state, index) {
        // console.log("mouse location:", e.clientX, e.clientY);
        setIndexMoving(state ? index : null);
        if (state) {
            const image = document.getElementById('adisplay-' + indexMoving)
            const rect = image.getBoundingClientRect();
        }
    }
    function handleMoveDisplay(e) {
        if (indexMoving == null) return;
        console.log(e.clientX, e.clientY)
        const image = document.getElementById('adisplay-' + indexMoving)
        const rect = image.getBoundingClientRect();
        positions[indexMoving] = { x: e.clientX - rect.width / 2, y: e.clientY - rect.height };
        setPositions([...positions]);
    }

    return (
        <div id={'display-container'} onMouseMoveCapture={handleMoveDisplay} onClick={() => startMoveDisplay(false, null)} style={indexMoving != null ? {cursor: 'grabbing'} : {}}>
            {monitors.map((mon, index) =>
                <Display data={mon} position={positions[index]} index={index} onMouseDown={startMoveDisplay} />
            )}
        </div>
    )
}
DisplayContainer.propTypes = {
    monitors: PropTypes.array.isrequired
}
export default DisplayContainer