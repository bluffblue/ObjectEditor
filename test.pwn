#include <a_samp>
#include "include/dynamic_object_editor"

public OnFilterScriptInit()
{
    new objectid = CreateDynamicObject(1337, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0);
    printf("Created dynamic object with ID: %d", objectid);
    return 1;
}

public OnPlayerCommandText(playerid, cmdtext[])
{
    if (!strcmp(cmdtext, "/editobject", true))
    {
        new Float:x, Float:y, Float:z;
        GetDynamicObjectPos(1, x, y, z);
        printf("Object position: %f, %f, %f", x, y, z);
        EditDynamicObject(playerid, 1);
        return 1;
    }
    return 0;
}

public OnPlayerEditObject(playerid, objectid, response, Float:x, Float:y, Float:z, Float:rx, Float:ry, Float:rz)
{
    SetDynamicObjectPos(objectid, x, y, z);
    SetDynamicObjectRot(objectid, rx, ry, rz);
    return 1;
}
