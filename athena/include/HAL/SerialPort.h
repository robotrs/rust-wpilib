/*----------------------------------------------------------------------------*/
/* Copyright (c) FIRST 2016. All Rights Reserved.                             */
/* Open Source Software - may be modified and shared by FRC teams. The code   */
/* must be accompanied by the FIRST BSD license file in the root directory of */
/* the project.                                                               */
/*----------------------------------------------------------------------------*/

#pragma once

#include <stdint.h>

enum HAL_SerialPort {
  HAL_SerialPort_Onboard = 0,
  HAL_SerialPort_MXP = 1,
  HAL_SerialPort_USB1 = 2,
  HAL_SerialPort_USB2 = 3
};

#ifdef __cplusplus
extern "C" {
#endif

void HAL_InitializeSerialPort(enum HAL_SerialPort port, int32_t* status);
void HAL_SetSerialBaudRate(enum HAL_SerialPort port, int32_t baud, int32_t* status);
void HAL_SetSerialDataBits(enum HAL_SerialPort port, int32_t bits, int32_t* status);
void HAL_SetSerialParity(enum HAL_SerialPort port, int32_t parity, int32_t* status);
void HAL_SetSerialStopBits(enum HAL_SerialPort port, int32_t stopBits,
                           int32_t* status);
void HAL_SetSerialWriteMode(enum HAL_SerialPort port, int32_t mode, int32_t* status);
void HAL_SetSerialFlowControl(enum HAL_SerialPort port, int32_t flow,
                              int32_t* status);
void HAL_SetSerialTimeout(enum HAL_SerialPort port, double timeout, int32_t* status);
void HAL_EnableSerialTermination(enum HAL_SerialPort port, char terminator,
                                 int32_t* status);
void HAL_DisableSerialTermination(enum HAL_SerialPort port, int32_t* status);
void HAL_SetSerialReadBufferSize(enum HAL_SerialPort port, int32_t size,
                                 int32_t* status);
void HAL_SetSerialWriteBufferSize(enum HAL_SerialPort port, int32_t size,
                                  int32_t* status);
int32_t HAL_GetSerialBytesReceived(enum HAL_SerialPort port, int32_t* status);
int32_t HAL_ReadSerial(enum HAL_SerialPort port, char* buffer, int32_t count,
                       int32_t* status);
int32_t HAL_WriteSerial(enum HAL_SerialPort port, const char* buffer, int32_t count,
                        int32_t* status);
void HAL_FlushSerial(enum HAL_SerialPort port, int32_t* status);
void HAL_ClearSerial(enum HAL_SerialPort port, int32_t* status);
void HAL_CloseSerial(enum HAL_SerialPort port, int32_t* status);
#ifdef __cplusplus
}
#endif
