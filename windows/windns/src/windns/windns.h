#pragma once
#include <cstdint>

//
// WINDNS public API
//

#ifdef WINDNS_EXPORTS
#define WINDNS_LINKAGE __declspec(dllexport)
#else
#define WINDNS_LINKAGE __declspec(dllimport)
#endif

#define WINDNS_API __stdcall

///////////////////////////////////////////////////////////////////////////////
// Functions
///////////////////////////////////////////////////////////////////////////////

enum WinDnsLogCategory
{
	WINDNS_LOG_CATEGORY_ERROR	= 0x01,
	WINDNS_LOG_CATEGORY_INFO	= 0x02
};

typedef void (WINDNS_API *WinDnsLogSink)(WinDnsLogCategory category, const char *message,
	const char **details, uint32_t numDetails, void *context);

//
// WinDns_Initialize:
//
// Call this function once at startup, to acquire resources etc.
// The error callback is OPTIONAL.
//
extern "C"
WINDNS_LINKAGE
bool
WINDNS_API
WinDns_Initialize(
	WinDnsLogSink logSink,
	void *logContext
);

//
// WinDns_Deinitialize:
//
// Call this function once before unloading WINDNS or exiting the process.
//
extern "C"
WINDNS_LINKAGE
bool
WINDNS_API
WinDns_Deinitialize(
);

//
// WinDns_Set:
//
// Configure DNS servers on given adapter.
//
extern "C"
WINDNS_LINKAGE
bool
WINDNS_API
WinDns_Set(
	const wchar_t *interfaceAlias,
	const wchar_t **ipv4Servers,
	uint32_t numIpv4Servers,
	const wchar_t **ipv6Servers,
	uint32_t numIpv6Servers
);
