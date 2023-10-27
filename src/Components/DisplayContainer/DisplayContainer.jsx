import React from 'react'
import Display from '../Display/Display'
import './DisplayContainer.scss'
import {PropTypes} from 'prop-types';

const DisplayContainer = ({monitors}) => {
  return (
    <div id={'display-container'}>
        {monitors.map((mon) => <Display/>)}
    </div>
  )
}
DisplayContainer.propTypes = {
    monitors: PropTypes.object.isrequired
}
export default DisplayContainer