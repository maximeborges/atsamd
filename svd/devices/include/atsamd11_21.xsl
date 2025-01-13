<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:template match = "/device/peripherals/peripheral[name='SERCOM0']/registers/cluster[name='USART']/register[name='CTRLA']/fields/field[name='FORM']">
    <xsl:copy>
      <xsl:copy-of select="@*"/>
      <xsl:copy-of select="node()"/>
      <enumeratedValues>
        <enumeratedValue>
          <name>NO_PARITY</name>
          <description>USART frame</description>
          <value>0x0</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>WITH_PARITY</name>
          <description>USART frame with parity</description>
          <value>0x1</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>LIN_CLIENT</name>
          <description>LIN Client with break detection and auto-baud</description>
          <value>0x4</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>LIN_CLIENT_WITH_PARITY</name>
          <description>LIN Client with parity, break detection and auto-baud</description>
          <value>0x5</value>
        </enumeratedValue>
      </enumeratedValues>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>
