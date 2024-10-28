<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:include href="./atsamd11-21-ptc.xsl"/>

  <!-- Add enumeratedValue for PTC in GCLK CLKCTRL register's ID field -->
  <xsl:template match="/device/peripherals/peripheral[name='GCLK']/registers/register[name='CLKCTRL']/fields/field[name='ID']/enumeratedValues">
    <xsl:call-template name="GCLK_ID">
      <xsl:with-param name="VALUE">0x17</xsl:with-param>
    </xsl:call-template>
  </xsl:template>

  <!-- Add PTC peripheral -->
  <xsl:template match="/device/peripherals">
    <xsl:call-template name="PTC">
      <xsl:with-param name="BASEADDRESS">0x42004C00</xsl:with-param>
      <xsl:with-param name="INTERRUPT_VALUE">18</xsl:with-param>
    </xsl:call-template>
  </xsl:template>
</xsl:stylesheet>