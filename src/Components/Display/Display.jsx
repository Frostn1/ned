import React from 'react'
import './Display.scss'
import { PropTypes } from 'prop-types'

const ratio = .1;

const Display = ({ data, position, index, onMouseDown }) => {
    const height = data.height * ratio;
    const width = data.width * ratio;
    const style = { width: `${width}px`, height: `${height}px`, top: `${position && position.y}px`, left: `${position && position.x || 0}px` }
    function onDragStart(e) {
        onMouseDown(true, index)
    }
    return (
        <div id={'adisplay-' + index} key={Math.random()} style={style} onMouseDownCapture={onDragStart}>
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