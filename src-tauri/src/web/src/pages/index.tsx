import React, { useCallback, useEffect, useState } from 'react';
import { invoke, promisified } from 'tauri/api/tauri';
import { Button } from 'antd';

import styled from 'styled-components';

export default () => {
  const [title, setTitle] = useState<string>('init');
  useEffect(() => {}, []);
  const handleInit = useCallback(() => {
    setTitle('init');
  }, []);

  const handleSyncClick = useCallback(() => {
    invoke({
      cmd: 'sendMessage',
      payload: {
        name: 'hello',
      },
    });
  }, []);

  const handleAsyncClick = useCallback(() => {
    promisified({
      cmd: 'sendAsyncMessage',
      payload: {
        name: 'async',
      },
    })
      .then((response: any) => {
        const { message } = response;
        console.log(message);
        setTitle(message);
      })
      .catch((error) => {
        console.error(error);
      });
  }, []);
  return (
    <div>
      <h1>{title}</h1>
      <Button onClick={handleSyncClick}>send message</Button>
      <Button onClick={handleAsyncClick}>send async message</Button>
      <Button onClick={handleInit}>init</Button>
    </div>
  );
};
