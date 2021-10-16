
#ifndef INCLUDE_atlier_nid_h__
#define INCLUDE_atlier_nid_h__

/** Size (in bytes) of a raw/binary nid */
#define ATLIER_NID_RAWSZ 64

/** Unique identity of any node */
typedef struct atlier_nid {
	/** raw binary formatted id */
	unsigned char id[ATLIER_NID_RAWSZ];
} atlier_nid;

#endif

