import React from 'react'
import './Display.scss'
import { PropTypes } from 'prop-types'

const ratio = .1;

const Display = ({ data, position, index, onMouseDown }) => {
    const height = data.height * ratio;
    const width = data.width * ratio;
    const style = { width: `${width}px`, height: `${height}px`, top: `${position && position.y}px`, left: `${position && position.x || 0}px`}
    console.log('position', position)
    function onDragStart (e) {
        console.log('drag start');
        onMouseDown(true, index)
    }
    function onDragStop (e) {
        console.log('drag stop');
        onMouseDown(false, index)
    }
    return (
        <div id={'display-'+index} key={Math.random()} style={style} onMouseDownCapture={onDragStart}>
            <div className={'resolution'}>
                {data.width} X {data.height}
            </div>
            <div className={'name'}>
                {data.name}
            </div>
        </div>
    )
}

Display.propTypes = {
    data: PropTypes.object.isrequired,
    position: PropTypes.object.isrequired,
    index: PropTypes.number.isrequired,
    onMouseDown: PropTypes.func.isrequired
}
export default Display