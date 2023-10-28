import React from 'react'
import './Header.scss';
import CommandHeader
  from '../CommandHeader/CommandHeader';

const Header = () => {
  return (
    <div id={'header'}>
      NED
      <CommandHeader />
    </div>
  )
}

export default Header