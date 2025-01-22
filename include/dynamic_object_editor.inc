#if defined _inc_dynamic_object_editor
    #undef _inc_dynamic_object_editor
#endif

#if defined _dynamic_object_editor_included
    #endinput
#endif
#define _dynamic_object_editor_included

native CreateDynamicObject(modelid, Float:x, Float:y, Float:z, Float:rx, Float:ry, Float:rz);
native EditDynamicObject(playerid, objectid);
native SaveDynamicObject(objectid);
native Float:GetDynamicObjectPos(objectid, &Float:x, &Float:y, &Float:z);
native Float:GetDynamicObjectRot(objectid, &Float:rx, &Float:ry, &Float:rz);
native SetDynamicObjectPos(objectid, Float:x, Float:y, Float:z);
native SetDynamicObjectRot(objectid, Float:rx, Float:ry, Float:rz);
native DeleteDynamicObject(objectid);
native ExportMapData(const filename[]);
native ImportMapData(const filename[]);
